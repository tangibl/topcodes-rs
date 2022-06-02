use topcodes::scanner::Scanner;

use image::io::Reader as ImageReader;

fn main() {
    let mut scanner = {
        let img = ImageReader::open("assets/photo.png")
            .unwrap()
            .decode()
            .unwrap();
        let (width, height) = (img.width() as usize, img.height() as usize);
        let image_raw = img.into_rgb8().into_raw();
        let buffer = &image_raw;
        Scanner::new(buffer, width, height)
    };

    let topcodes = scanner.scan();
    println!("{:?}", topcodes);
}
