use criterion::{criterion_group, criterion_main, Criterion};
use image::io::Reader as ImageReader;
use topcodes::scanner::Scanner;

fn scan(scanner: &mut Scanner, buffer: &[u8]) {
    let topcodes = scanner.scan(buffer).unwrap();
    assert_eq!(3, topcodes.len());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Scanner (photo)", |b| {
        b.iter_batched(
            || {
                let img = ImageReader::open("assets/photo.png")
                    .unwrap()
                    .decode()
                    .unwrap();
                let (width, height) = (img.width() as usize, img.height() as usize);
                let buffer = img.into_rgb8().into_raw();
                (Scanner::new(width, height), buffer)
            },
            |(mut scanner, buffer)| scan(&mut scanner, &buffer),
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function("Scanner (source)", |b| {
        b.iter_batched(
            || {
                let img = ImageReader::open("assets/source.png")
                    .unwrap()
                    .decode()
                    .unwrap();
                let (width, height) = (img.width() as usize, img.height() as usize);
                let buffer = img.into_rgb8().into_raw();
                (Scanner::new(width, height), buffer)
            },
            |(mut scanner, buffer)| scan(&mut scanner, &buffer),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
