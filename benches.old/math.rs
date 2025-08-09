use criterion::{criterion_group, criterion_main, Criterion};
use open_pql::{constants::N_RANKS, prim::math, Rank, Rank16};

fn gen() -> u8 {
    let r1 = fastrand::u8(0..N_RANKS);
    let r2 = fastrand::u8(0..N_RANKS);

    math::combinatorics::combination_of_2_ranks_to_index(
        Rank16::from(
            [Rank::ARR_ALL[r1 as usize], Rank::ARR_ALL[r2 as usize]].as_ref(),
        )
        .to_u16(),
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("math");

    group.bench_function("combination_of_2_ranks_to_index", |b| {
        b.iter(gen);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
