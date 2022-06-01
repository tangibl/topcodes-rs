use crate::scanner::Scanner;

#[cfg(feature = "visualize")]
use image::io::Reader as ImageReader;

mod scanner;
mod topcode;
mod utils;

fn main() {
    #[cfg(feature = "visualize")]
    {
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

        scanner.write_thresholding_image("before_thresholding");

        let topcodes = scanner.scan();

        println!("{:?}", topcodes);

        scanner.write_thresholding_image("after_thresholding");
    }

    #[cfg(not(feature = "visualize"))]
    {
        eprintln!("The run target only works with the 'visualize' feature enabled. Use `cargo run --feature visualize` instead.'");
    }
}
