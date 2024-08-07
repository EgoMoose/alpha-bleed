from PIL import Image, ImageDraw

scale = 100

pixels = [
    [(255, 0, 0, 255, 1), (0, 255, 0, 255, 1)],
]

img = Image.new(mode="RGBA", size=(len(pixels[0]) * scale, len(pixels) * scale))
draw = ImageDraw.Draw(img)

def checkerboard(box_size):
    check = img.copy()
    w, h = check.width, check.height
    loaded = check.load() # create the pixel map
    drawc = ImageDraw.Draw(check)

    for i in range (0, h, box_size):
        for j in range(0, w, box_size):
            y, x = i // box_size, j // box_size
            color = (30, 30, 30, 255) if (y&1)^(x&1) else (20, 20, 20, 255)
            for di in range(box_size):
                for dj in range(box_size):
                    loaded[min(j+dj, w - 1), min(i+di, h - 1)] = color

    for (y, row) in enumerate(pixels):
        for (x, pixel) in enumerate(row):
            if pixel[4] == 0:
                x1 = x * scale
                x2 = x1 + scale
                y1 = y * scale
                y2 = y1 + scale

                drawc.rectangle((x1, y1, x2, y2), fill=(0, 0, 0, 0)) 
    
    return check

for (y, row) in enumerate(pixels):
    for (x, pixel) in enumerate(row):
        x1 = x * scale
        x2 = x1 + scale
        y1 = y * scale
        y2 = y1 + scale

        draw.rectangle((x1, y1, x2, y2), fill=pixel[:4]) 

Image.alpha_composite(checkerboard(8), img).save("output.png")