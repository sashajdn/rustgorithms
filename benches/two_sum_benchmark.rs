#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustgorithms::arrays::two_sum;

fn bench_two_sum(c: &mut Criterion) {
    let nums = &[2, 7, 11, 15, 19, 21, 30];
    let target = 26;

    c.bench_function("two_sum", |b| {
        b.iter(|| {
            // The black_box function is used to prevent compiler optimizations from disregarding
            // unused computations.
            two_sum::two_sum(black_box(nums), black_box(target))
        });
    });
}

criterion_group!(benches, bench_two_sum);
criterion_main!(benches);
