use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use openpql_prelude::{Game, HandRating, c64};

const DATA: [(&str, &str); 20] = [
    ("Js Ts 9s 8s", "As Ks Qs Th Jh"), // StraightFlush(A)
    ("As 5s 9s 8s", "2s 3s 4s Th Jh"), // StraightFlush(5)
    ("As Ah Ks Kh", "Ac Ad Kc Kd Qs"), // Quads(A, K)
    ("Qs Qh Qc 7s", "7h 7c 7d Qd As"), // Quads(7, Q)
    ("Ac Kc 7s 7h", "7c Ks kh As Ah"), // FullHouse(A, K)
    ("7s 7h 8s 8h", "7c 6s 6h As Ah"), // FullHouse(7, A)
    ("Qs Qh 8s 8h", "7h 7c 7d As Ah"), // FullHouse(7, Q)
    ("As Ks 2s 3s", "Js Ts 9s 2h 3h"), // Flush(AKJT9, )
    ("Js Th 9d 8c", "As Kh Qd Tc Jc"), // Straight(A)
    ("As 5h 9d 8c", "2s 3h 4d Tc Jc"), // Straight(5)
    ("Ac Ks Kh 2s", "7c Ts Th As Ah"), // Trips(A, KT)
    ("Ks Qh 8s 8h", "Kh Qc 8c 2s 3h"), // Trips(8, KQ)
    ("Ks Qh 8s 9h", "7h 7c 7d As Ah"), // Trips(7, KQ)
    ("Ks Ah 8d 8c", "7s 7h Qs 2h 3h"), // TwoPair(78, Q)
    ("8s Qh 2s 3s", "7h 7c 8d As Kh"), // TwoPair(78, Q)
    ("7s 8s As Ks", "7h 8h Qs 2h 3h"), // TwoPair(78, Q)
    ("7s 7h 4c 5d", "As Kh Qc Jd Ts"), // Pair(7, AKQ)
    ("3h 4c 5d 7h", "7s As Kh Qc 2s"), // Pair(7, AK5)
    ("2s 3h 4c 5d", "7s 7h As Kh Qc"), // Pair(7, A45)
    ("2s 6s 7h 8c", "As Kh Qc Jd Ts"), // HighCard(AKQ78)
];

fn run(p: &str, b: &str) -> HandRating {
    Game::Omaha.eval_rating(c64!(p), c64!(b))
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("omaha");

    group.bench_function("9 cards", |b| {
        b.iter(|| {
            for (p, b) in DATA {
                run(black_box(p), black_box(b));
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
