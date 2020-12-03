use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2020::days::day1::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut input = load_input();
    c.bench_function("day 1 part one", |b| b.iter(|| load_input));
    c.bench_function("day 1 part one", |b| b.iter(|| one(input)));
    c.bench_function("day 1 part two", |b| b.iter(|| two(input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);