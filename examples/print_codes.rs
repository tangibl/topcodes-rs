use topcodes::scanner::Scanner;

use image::io::Reader as ImageReader;

fn main() {
    let (mut scanner, buffer) = {
        let img = ImageReader::open("assets/photo.png")
            .unwrap()
            .decode()
            .unwrap();
        let (width, height) = (img.width() as usize, img.height() as usize);
        let buffer = img.into_rgb8().into_raw();
        (Scanner::new(width, height), buffer)
    };

    let topcodes = scanner.scan(&buffer);
    println!("{:?}", topcodes);
}
