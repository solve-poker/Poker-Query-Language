//use std::time::Duration;

//use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use open_pql::{range_checker::Checker, Card};
//
//fn gen(n: u8, s: &str) -> Vec<Vec<Card>> {
//    Checker::from_src(s, n).unwrap().generate()
//}
//
//fn criterion_benchmark(c: &mut Criterion) {
//    let mut group = c.benchmark_group("generate");
//
//    //group.sample_size(10);
//    //group.warm_up_time(Duration::from_millis(200));
//
//    group.bench_function("A[K,T][43-]", |b| {
//        b.iter(|| gen(black_box(4), black_box("A[K,T][43-]")));
//    });
//
//    group.bench_function("ABTsw", |b| {
//        b.iter(|| gen(black_box(4), black_box("ABTsw")));
//    });
//
//    group.bench_function("AsKsQsB", |b| {
//        b.iter(|| gen(black_box(4), black_box("AsKsQsB")));
//    });
//
//    group.bench_function("****", |b| {
//        b.iter(|| gen(black_box(4), black_box("****")));
//    });
//}
//
//criterion_group!(benches, criterion_benchmark);
//criterion_main!(benches);
