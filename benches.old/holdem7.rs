use criterion::{black_box, criterion_group, criterion_main, Criterion};
use open_pql::{c64, eval_holdem7, eval_omaha9, eval_shortdeck7, PQLHiRating};

fn e_h(s: &str) -> PQLHiRating {
    eval_holdem7(c64!(s))
}

fn e_s(s: &str) -> PQLHiRating {
    eval_shortdeck7(c64!(s))
}

fn e_o(s: &str) -> PQLHiRating {
    let mut s = s.split('|');
    eval_omaha9(c64!(s.next().unwrap()), c64!(s.next().unwrap()))
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluate");

    group.bench_function("holdem7", |b| {
        b.iter(|| {
            e_h(black_box("As Ah Ad Kd Qd Jd Td")); // (RoyalFlush)
            e_h(black_box("7h 8d 6s 6h 6c 6d Ts")); // (Quads, 6, T)
            e_h(black_box("As Ah Ts Th Td 9s 9h")); // (FullHouse, T, A)
            e_h(black_box("Ah Ad 6s 7s 8s 9s Js")); // (Flush, 6789J)
            e_h(black_box("9s 8h As Kh Qd Jc Ts")); // (Straight, A)
            e_h(black_box("Qd 6c Ts Th Td As Kh")); // (Trips, T, KA)
            e_h(black_box("Ts Th 6s 6h Ks Qh Jd")); // (TwoPair, 6T, K)
            e_h(black_box("Js Jh 6s 7h 8d 9c Ks")); // (Pair, J, 89K)
            e_h(black_box("6s 7h 8d 9c Js Qh Kd")); // (HighCard, 89JQK)
        });
    });

    group.bench_function("shortdeck7", |b| {
        b.iter(|| {
            e_s(black_box("7s 8s 9s Ts Js Qs Ks")); // (StraightFlush, K)
            e_s(black_box("7s 7h 7d 7c Qs Kh Kd")); // (Quads, 7, K)
            e_s(black_box("Ah Ad 6s 7s 8s 9s Js")); // (Flush, 6789J)
            e_s(black_box("7s 7h 7d Tc Qs Kh Kd")); // (FullHouse, 7, K)
            e_s(black_box("7s 8h 9d Tc Js Qh Kd")); // (Straight, K)

            e_s(black_box("7s 7h 9d Tc Qs Kh 7d")); // (Trips, 7, QK)
            e_s(black_box("7s 7h 9d Tc Qs Kh Kd")); // (TwoPair, 7K, Q)
            e_s(black_box("7s 8h 9d Tc Qs Kh Kd")); // (Pair, K, 9TQ)
            e_s(black_box("7s 8h 9d Jc Qs Kh Ad")); // (HighCard, 9JQKA)
        });
    });

    group.bench_function("omaha9", |b| {
        b.iter(|| {
            e_o(black_box("6s 7s 8s 9s | As Ks Qs Js Ts")); // (StraightFlush, Q)
            e_o(black_box("6s 7h Ad Ac | As Ah Qd Jc Ts")); // (Quads, A, Q)
            e_o(black_box("6s 7h Kd Kc | As Ah Ad Jc Ts")); // (FullHouse, A, K)
            e_o(black_box("6s 7s 8s 9s | As Ks Qs Js 2s")); // (Flush, 89QKA)

            e_o(black_box("6s 7h 8d 9c | As Kh Qd Jc Ts")); // (Straight, Q)
            e_o(black_box("6s 7h Ad Kc | As Ah Qd Jc 2s")); // (Trips, A, QK)
            e_o(black_box("6s 7h Kd Kc | As Ah Qd Jc 2s")); // (TwoPair, KA, Q)
            e_o(black_box("6s 7h 8d Ac | As Kh Qd Jc 2s")); // (Pair, A, 8QK)
            e_o(black_box("6s 7h 8d 9c | As Kh Qd Jc 2s")); // (HighCard, 89QKA)
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
