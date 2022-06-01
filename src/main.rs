use crate::scanner::Scanner;

mod scanner;
mod topcode;
mod utils;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let buffer: [u8; WIDTH * HEIGHT * 3] = [0; WIDTH * HEIGHT * 3];
    let mut scanner = Scanner::new(&buffer, WIDTH, HEIGHT);
    let topcodes = scanner.scan();

    println!("{:?}", topcodes);

    scanner.write_thresholding_ppm("test");
}
