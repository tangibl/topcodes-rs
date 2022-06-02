use criterion::{criterion_group, criterion_main, Criterion};
use image::io::Reader as ImageReader;
use topcodes::scanner::Scanner;

fn scan(buffer: &[u8], width: usize, height: usize) {
    let mut scanner = Scanner::new(buffer, width, height);
    let topcodes = scanner.scan();
    assert_eq!(3, topcodes.len());
}

fn criterion_benchmark(c: &mut Criterion) {
    let img = ImageReader::open("assets/photo.png")
        .unwrap()
        .decode()
        .unwrap();
    let (width, height) = (img.width() as usize, img.height() as usize);
    let image_raw = img.into_rgb8().into_raw();
    let buffer = &image_raw;
    c.bench_function("Scanner (photo)", |b| {
        b.iter(|| scan(buffer, width, height))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
