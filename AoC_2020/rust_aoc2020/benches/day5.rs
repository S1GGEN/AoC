use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2020::days::day5::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = load_input("input");
    c.bench_function("day 5 load input", |b| b.iter(|| load_input("input")));
    c.bench_function("day 5 both parts", |b| b.iter(|| both(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);