use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc2020::days::day09::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = load_input("input");
    let part_one_result = calculate_one(&input);
    c.bench_function("day 9 load input", |b| b.iter(|| load_input("input")));
    c.bench_function("day 9 part one", |b| b.iter(|| one(&input)));
    c.bench_function("day 9 part two including part one to find target", |b| b.iter(|| two(&input, None)));
    c.bench_function("day 9 part two with target included", |b| b.iter(|| two(&input, part_one_result)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);