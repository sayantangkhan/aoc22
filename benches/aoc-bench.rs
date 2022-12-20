use aoc22::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::env;
use std::path::Path;

fn criterion_day_6_1_benchmark(c: &mut Criterion) {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("inputs/day06")
        .into_os_string()
        .into_string()
        .unwrap();
    c.bench_function("Day 6: Part 1 benchmark", |b| b.iter(|| day_6_1(&filename)));
}

fn criterion_day_6_2_benchmark(c: &mut Criterion) {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("inputs/day06")
        .into_os_string()
        .into_string()
        .unwrap();
    c.bench_function("Day 6: Part 2 benchmark", |b| b.iter(|| day_6_1(&filename)));
}

criterion_group!(
    benches,
    criterion_day_6_1_benchmark,
    criterion_day_6_2_benchmark
);
criterion_main!(benches);
