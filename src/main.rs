use crate::scanner::Scanner;

mod scanner;
mod topcode;
mod utils;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let buffer: [u8; WIDTH * HEIGHT * 3] = [0; WIDTH * HEIGHT * 3];
    let scanner = Scanner::new(&buffer, WIDTH, HEIGHT);

    scanner.write_thresholding_ppm("test");
}
