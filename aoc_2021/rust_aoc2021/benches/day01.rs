use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2021::days::day01::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = load_input("input");
    c.bench_function("day 01 load input", |b| b.iter(|| load_input("input")));
    c.bench_function("day 01 part one", |b| b.iter(|| one(&input)));
    c.bench_function("day 01 part two", |b| b.iter(|| two(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
