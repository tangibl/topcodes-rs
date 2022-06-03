use topcodes::scanner::Scanner;

use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};

fn main() {
    println!("Loading image...");
    let mut img = ImageReader::open("assets/photo.png")
        .unwrap()
        .decode()
        .unwrap();
    let (width, height) = (img.width() as usize, img.height() as usize);
    let buffer = img.clone().into_rgb8().into_raw();

    println!("Generating scanner buffer...");
    let mut scanner = Scanner::new(width, height);

    println!("Scanning TopCodes...");
    let topcodes = scanner.scan(&buffer);

    println!("Found {} TopCodes.", topcodes.len());

    for code in &topcodes {
        // Draw circle
        draw_circle(
            &mut img,
            code.x as usize,
            code.y as usize,
            code.radius() as usize,
            [255, 170, 0, 80],
        );
        // Draw blue rectangle for orientation
        let x = code.orientation.cos() * code.radius() + code.x;
        let y = code.orientation.sin() * code.radius() + code.y;
        draw_rect(
            &mut img,
            x as usize - 2,
            y as usize - 2,
            4,
            4,
            [0, 127, 255, 80],
        );
        // Draw center as a small red rectangle
        draw_rect(
            &mut img,
            code.x as usize - 2,
            code.y as usize - 2,
            4,
            4,
            [255, 0, 0, 80],
        );
    }

    if topcodes.len() == 0 {
        println!("Aborting as no TopCodes were found.");
    } else {
        println!("Saving image...");
        img.save("target/annotated.png")
            .expect("Failed to save annotated image");
    }
}

fn draw_rect(
    img: &mut DynamicImage,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: [u8; 4],
) {
    for i in 0..=width {
        for j in 0..=height {
            let rx = (x + i) as u32;
            let ry = (y + j) as u32;
            let pixel = img.get_pixel(rx, ry);
            img.put_pixel(rx, ry, blend(pixel, Rgba(color)));
        }
    }
}

fn draw_circle(img: &mut DynamicImage, x: usize, y: usize, radius: usize, color: [u8; 4]) {
    let radius: isize = radius as isize;
    let radius_squared = radius * radius;
    let x = x as isize;
    let y = y as isize;
    for i in -radius..=radius {
        for j in -radius..=radius {
            if (i * i + j * j) < radius_squared {
                let cx = (x + i) as u32;
                let cy = (y + j) as u32;
                let pixel = img.get_pixel(cx, cy);
                img.put_pixel(cx, cy, blend(pixel, Rgba(color)));
            }
        }
    }
}

fn blend(color: Rgba<u8>, other: Rgba<u8>) -> Rgba<u8> {
    let a = other[3] as f32 / 255.0;
    let a_inverse = 1.0 - a;

    let r = (color[0] as f32 * a_inverse + other[0] as f32 * a) as u8;
    let g = (color[1] as f32 * a_inverse + other[1] as f32 * a) as u8;
    let b = (color[2] as f32 * a_inverse + other[2] as f32 * a) as u8;

    Rgba([r, g, b, 255])
}
