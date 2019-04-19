use criterion::{criterion_group, criterion_main, Criterion};

use scottish_names::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("surname", |b| b.iter(|| surname()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
