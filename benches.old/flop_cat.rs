use criterion::{black_box, criterion_group, criterion_main, Criterion};
use open_pql::{functions::flop_hand_category, *};

fn e_h(s: &str) -> FlopHandCategory {
    let mut s = s.split('|');

    flop_hand_category(
        PQLGame::Holdem,
        cards!(s.next().unwrap()).as_ref(),
        flop!(s.next().unwrap()),
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("flopcategory");

    group.bench_function("holdem", |b| {
        b.iter(|| {
            e_h(black_box("As Ks | Qs Js Ts")); // StraightFlush
            e_h(black_box("As Ah | Ad Ac Ks")); // Quads
            e_h(black_box("As Ah | Ad Kc Ks")); // FullHouse
            e_h(black_box("As Ks | Qs Js 9s")); // Flush
            e_h(black_box("As Kh | Qd Jc Ts")); // Straight

            e_h(black_box("As Ah | Ad Kc Qs")); // Set
            e_h(black_box("As 2h | Ad Ac Qs")); // Trips

            e_h(black_box("Js Qh | Td Jc Qs")); // TopTwo
            e_h(black_box("Ts Qh | Td Jc Qs")); // TopAndBottom
            e_h(black_box("Js Th | Td Jc Qs")); // BottomTwo

            e_h(black_box("As Ah | Kd Qc Js")); // Overpair
            e_h(black_box("Ks 2h | Kd Qc Js")); // TopPair
            e_h(black_box("Qs Qh | Kd Tc 7s")); // Pocket12
            e_h(black_box("Ts 2h | Kd Tc 7s")); // SecondPair
            e_h(black_box("9s 9h | Kd Tc 7s")); // Pocket23
            e_h(black_box("7s 2h | Kd Tc 7s")); // ThirdPair
            e_h(black_box("2s 2h | Kd Tc 7s")); // UnderPair

            e_h(black_box("As Kh | Qd Jc 9s")); // Nothing
        });
    });

    group.bench_function("omaha", |b| {
        b.iter(|| {
            e_h(black_box("3d 6c As Ks | Qs Js Ts")); // StraightFlush
            e_h(black_box("3d 6c As Ah | Ad Ac Ks")); // Quads
            e_h(black_box("3d 6c As Ah | Ad Kc Ks")); // FullHouse
            e_h(black_box("3d 6c As Ks | Qs Js 9s")); // Flush
            e_h(black_box("3d 6c As Kh | Qd Jc Ts")); // Straight

            e_h(black_box("3d 6c As Ah | Ad Kc Qs")); // Set
            e_h(black_box("3d 6c As 2h | Ad Ac Qs")); // Trips

            e_h(black_box("3d 6c Js Qh | Td Jc Qs")); // TopTwo
            e_h(black_box("3d 6c Ts Qh | Td Jc Qs")); // TopAndBottom
            e_h(black_box("3d 6c Js Th | Td Jc Qs")); // BottomTwo

            e_h(black_box("3d 6c As Ah | Kd Qc Js")); // Overpair
            e_h(black_box("3d 6c Ks 2h | Kd Qc Js")); // TopPair
            e_h(black_box("3d 6c Qs Qh | Kd Tc 7s")); // Pocket12
            e_h(black_box("3d 6c Ts 2h | Kd Tc 7s")); // SecondPair
            e_h(black_box("3d 6c 9s 9h | Kd Tc 7s")); // Pocket23
            e_h(black_box("3d 6c 7s 2h | Kd Tc 7s")); // ThirdPair
            e_h(black_box("3d 6c 2s 2h | Kd Tc 7s")); // UnderPair

            e_h(black_box("3d 6c As Kh | Qd Jc 9s")); // Nothing
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
