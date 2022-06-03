use topcodes::scanner::Scanner;

use image::{io::Reader as ImageReader, DynamicImage, GenericImage, Rgba};

fn main() {
    let mut img = ImageReader::open("assets/photo.png")
        .unwrap()
        .decode()
        .unwrap();
    let (width, height) = (img.width() as usize, img.height() as usize);
    let buffer = img.clone().into_rgb8().into_raw();
    let mut scanner = Scanner::new(width, height);

    let topcodes = scanner.scan(&buffer);

    for code in topcodes {
        // Draw blue rectangle for orientation
        let x = code.orientation.cos() * code.radius() + code.x;
        let y = code.orientation.sin() * code.radius() + code.y;
        draw_rect(
            &mut img,
            x as usize - 2,
            y as usize - 2,
            4,
            4,
            [0, 127, 255, 170],
        );
        // Draw center as a small red rectangle
        draw_rect(
            &mut img,
            code.x as usize - 2,
            code.y as usize - 2,
            4,
            4,
            [255, 0, 0, 170],
        );
    }

    img.save("target/annotated.png")
        .expect("Failed to save annotated image");
}

fn draw_rect(
    img: &mut DynamicImage,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: [u8; 4],
) {
    for i in 0..width {
        for j in 0..height {
            img.blend_pixel((x + i) as u32, (y + j) as u32, Rgba(color));
        }
    }
}
