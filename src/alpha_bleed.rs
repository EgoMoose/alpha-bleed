use image::{Rgba, RgbaImage};
use std::collections::VecDeque;

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn alpha_bleed(image: &mut RgbaImage, thickness: usize) {
    let (width, height) = image.dimensions();

    let mut visited = vec![false; (width * height) as usize];
    let mut can_sample = vec![false; (width * height) as usize];
    let mut pixel_queue: VecDeque<(u32, u32)> = VecDeque::new();

    let adjacent_neighbors = |x, y| {
        NEIGHBOR_OFFSETS
            .iter()
            .filter_map(move |(x_offset, y_offset)| {
                let x_neighbor = (x as i32) + x_offset;
                let y_neighbor = (y as i32) + y_offset;

                if x_neighbor < 0
                    || x_neighbor >= (width as i32)
                    || y_neighbor < 0
                    || y_neighbor >= (height as i32)
                {
                    return None;
                }

                Some((x_neighbor as u32, y_neighbor as u32))
            })
    };

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let flat_index = (y * width + x) as usize;

            if pixel[3] != 0 {
                visited[flat_index] = true;
                can_sample[flat_index] = true;
                continue;
            }

            let has_opaque_neighbor = adjacent_neighbors(x, y).any(|(x_neighbor, y_neighbor)| {
                let neighbor_pixel = image.get_pixel(x_neighbor, y_neighbor);
                neighbor_pixel[3] != 0
            });

            if has_opaque_neighbor {
                visited[flat_index] = true;
                pixel_queue.push_back((x, y));
            }

            image.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    for _ in 0..thickness {
        let queue_length = pixel_queue.len();
        if queue_length == 0 {
            break;
        }

        let mut mutated_pixels: Vec<usize> = vec![0; queue_length];
        for _ in 0..queue_length {
            if let Some((x, y)) = pixel_queue.pop_front() {
                let flat_index = (y * width + x) as usize;

                let mut color = (0, 0, 0);
                let mut contributing = 0;

                for (x_neighbor, y_neighbor) in adjacent_neighbors(x, y) {
                    let flat_neighbor_index = (y_neighbor * width + x_neighbor) as usize;

                    if can_sample[flat_neighbor_index] {
                        let neighbor_pixel = image.get_pixel(x_neighbor, y_neighbor);

                        color.0 += neighbor_pixel[0] as u16;
                        color.1 += neighbor_pixel[1] as u16;
                        color.2 += neighbor_pixel[2] as u16;

                        contributing += 1;
                    } else if !visited[flat_neighbor_index] {
                        visited[flat_neighbor_index] = true;
                        pixel_queue.push_back((x_neighbor, y_neighbor));
                    }
                }

                let denominator = u16::max(1, contributing);
                let new_pixel = Rgba([
                    (color.0 / denominator) as u8,
                    (color.1 / denominator) as u8,
                    (color.2 / denominator) as u8,
                    0,
                ]);

                image.put_pixel(x, y, new_pixel);
                mutated_pixels.push(flat_index);
            }
        }

        for _ in 0..queue_length {
            if let Some(flat_index) = mutated_pixels.pop() {
                can_sample[flat_index] = true;
            }
        }
    }
}

#[allow(dead_code)]
pub fn make_opaque(image: &mut RgbaImage) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            if pixel[3] != 255 {
                let opaque_pixel = Rgba([pixel[0], pixel[1], pixel[2], 255]);
                image.put_pixel(x, y, opaque_pixel)
            }
        }
    }
}
