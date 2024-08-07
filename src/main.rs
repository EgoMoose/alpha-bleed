mod alpha_bleed;

use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let input_path = PathBuf::from(&args[0]);
    let output_path = PathBuf::from(&args[1]);

    match image::open(&input_path) {
        Err(e) => eprintln!("Error: {e}"),
        Ok(image) => {
            let mut rgba8_image = image.to_rgba8();

            alpha_bleed::alpha_bleed(&mut rgba8_image, 1);
            //alpha_bleed::make_opaque(&mut rgba8_image);

            match rgba8_image.save(&output_path) {
                Ok(_) => println!("Successfully saved ./{}", &args[1]),
                Err(_) => println!("Failed to save ./{}", &args[1]),
            }
        }
    }
}
