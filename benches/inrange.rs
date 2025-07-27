use std::io;

use criterion::{criterion_group, criterion_main, Criterion};
use open_pql::StatementsRunner;

fn gen() {
    let src = "
        select count(boardinrange('****2s')) from board='*'
    ";

    let mut r = StatementsRunner::new(
        src,
        60000,
        4,
        Box::new(io::Cursor::<Vec<u8>>::default()),
    );

    r.run();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("runner");

    group.bench_function("inrange", |b| {
        b.iter(gen);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
