

#[cfg(feature = "visualize")]
use image::io::Reader as ImageReader;

fn main() {
    #[cfg(feature = "visualize")]
    {
        let (mut scanner, buffer) = {
            let img = ImageReader::open("assets/photo.png")
                .unwrap()
                .decode()
                .unwrap();
            let (width, height) = (img.width() as usize, img.height() as usize);
            let buffer = img.into_rgb8().into_raw();
            (Scanner::new(width, height), buffer)
        };

        let _topcodes = scanner.scan(&buffer, |buffer, index| {
            (
                buffer[index * 3] as u32,
                buffer[index * 3 + 1] as u32,
                buffer[index * 3 + 2] as u32,
            )
        });
        scanner.write_thresholding_image("target/thresholded.png");
    }

    #[cfg(not(feature = "visualize"))]
    {
        eprintln!("The run target only works with the 'visualize' feature enabled. Use `cargo run --feature visualize` instead.'");
    }
}
