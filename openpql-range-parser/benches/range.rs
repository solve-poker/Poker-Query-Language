use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use openpql_prelude::HandN;
use openpql_range_parser::RangeChecker;

fn run(s: &str) -> usize {
    const SD: bool = false;
    let mut count = 0;
    let checker = RangeChecker::<4, SD>::from_src(s).unwrap();
    for cards in HandN::<4>::iter_all::<SD>() {
        if checker.is_satisfied(cards.as_slice()) {
            count += 1;
        }
    }
    count
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("range 4");

    group.bench_function("A[K,T][43-]", |b| {
        b.iter(|| run(black_box("A[K,T][43-]")));
    });

    group.bench_function("ABTsw", |b| {
        b.iter(|| run(black_box("ABTsw")));
    });

    group.bench_function("AsKsQsB", |b| {
        b.iter(|| run(black_box("AsKsQsB")));
    });

    group.bench_function("****", |b| {
        b.iter(|| run(black_box("****")));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
