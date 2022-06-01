use crate::scanner::Scanner;

use image::io::Reader as ImageReader;

mod scanner;
mod topcode;
mod utils;

fn main() {
    let mut scanner = {
        let img = ImageReader::open("assets/source.png")
            .unwrap()
            .decode()
            .unwrap();
        let (width, height) = (img.width() as usize, img.height() as usize);
        let image_raw = img.into_rgb8().into_raw();
        let buffer = &image_raw;
        Scanner::new(buffer, width, height)
    };

    #[cfg(feature = "visualize")]
    scanner.write_thresholding_image("before_thresholding");

    let topcodes = scanner.scan();

    println!("{:?}", topcodes);

    #[cfg(feature = "visualize")]
    scanner.write_thresholding_image("after_thresholding");
}
