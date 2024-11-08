use criterion::{black_box, criterion_group, criterion_main, Criterion};
use holo::holo::HolomorphicLookup;
use holo::parsing::parse_expression;
use image::GenericImageView;

fn benchmark_apply(c: &mut Criterion) {
    let img = image::open("images/input/dresden.jpg")
        .expect("Failed to load image")
        .to_rgb8();

    let (_, holomorphic_fn) = parse_expression("z^7 + z^5").unwrap();
    let (width, height) = img.dimensions();

    let lookup = HolomorphicLookup::new(holomorphic_fn, width, height);

    c.bench_function("apply", |b| {
        b.iter(|| {
            black_box(lookup.apply(&img));
        })
    });
}

criterion_group!(benches, benchmark_apply);
criterion_main!(benches);
