use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2020::days::day08::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = load_input("input");
    c.bench_function("day 08 load input", |b| b.iter(|| load_input("input")));
    c.bench_function("day 08 part one", |b| b.iter(|| one(&input)));
    c.bench_function("day 08 part two", |b| b.iter(|| two(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
