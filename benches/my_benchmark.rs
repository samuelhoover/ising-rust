use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ising::ising;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ising", |b| b.iter(|| ising::run(black_box(10.0), false)));
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(90));
    targets = criterion_benchmark
}
criterion_main!(benches);
