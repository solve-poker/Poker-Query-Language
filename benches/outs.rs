use criterion::{black_box, criterion_group, criterion_main, Criterion};
use open_pql::{functions::outs_to_hand_type, *};

fn e_h(s: &str) -> PQLCardCount {
    let mut s = s.split('|');

    let game = PQLGame::Holdem;

    outs_to_hand_type(
        cards!(s.next().unwrap()).as_ref(),
        PQLStreet::Flop,
        (HandType::Trips, game).into(),
        (game, board!(s.next().unwrap()), Card64::default().into()),
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("outstohandtype");

    group.bench_function("holdem", |b| {
        b.iter(|| {
            e_h(black_box("Qs Qh | Kd Tc 7s 2s 3s"));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
