use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use shrs::prelude::*;

fn construct_benchmark(c: &mut Criterion) {
    c.bench_function("construct", |b: &mut Bencher| {
        b.iter(|| black_box(ShellConfigBuilder::default().build().unwrap()))
    });
}

criterion_group!(benches, construct_benchmark);
criterion_main!(benches);
