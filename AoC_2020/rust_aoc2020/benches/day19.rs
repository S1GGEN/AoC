use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2020::days::day19::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_1 = load_input("input_1");
    let input_2 = load_input("input_2");
    c.bench_function("day 19 load input for part one", |b| {
        b.iter(|| load_input("input_1"))
    });
    c.bench_function("day 19 part one", |b| b.iter(|| one(&input_1)));
    c.bench_function("day 19 load input for part two", |b| {
        b.iter(|| load_input("input_2"))
    });
    c.bench_function("day 19 part two ", |b| b.iter(|| two(&input_2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
