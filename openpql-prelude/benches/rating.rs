use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use openpql_prelude::{Card, Card64, Game};

const N_HANDS: usize = 1000;

fn deal<const SD: bool>(rng: &mut fastrand::Rng, n: usize) -> Vec<Card> {
    let mut cards = Card::all::<SD>().to_vec();
    rng.shuffle(&mut cards);
    cards.truncate(n);
    cards
}

fn to_c64(cards: &[Card]) -> Card64 {
    cards
        .iter()
        .fold(Card64::default(), |acc, &c| acc | Card64::from(c))
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = fastrand::Rng::with_seed(0);

    let holdem: Vec<Card64> = (0..N_HANDS)
        .map(|_| to_c64(&deal::<false>(&mut rng, 7)))
        .collect();

    let shortdeck: Vec<Card64> = (0..N_HANDS)
        .map(|_| to_c64(&deal::<true>(&mut rng, 7)))
        .collect();

    let omaha: Vec<(Card64, Card64)> = (0..N_HANDS)
        .map(|_| {
            let cs = deal::<false>(&mut rng, 9);
            (to_c64(&cs[..4]), to_c64(&cs[4..]))
        })
        .collect();

    let omaha5: Vec<(Card64, Card64)> = (0..N_HANDS)
        .map(|_| {
            let cs = deal::<false>(&mut rng, 10);
            (to_c64(&cs[..5]), to_c64(&cs[5..]))
        })
        .collect();

    let mut group = c.benchmark_group("rating");

    group.bench_function("holdem 7 cards", |b| {
        b.iter(|| {
            for &h in &holdem {
                black_box(Game::Holdem.eval_rating(black_box(h), Card64::default()));
            }
        });
    });

    group.bench_function("shortdeck 7 cards", |b| {
        b.iter(|| {
            for &h in &shortdeck {
                black_box(Game::ShortDeck.eval_rating(black_box(h), Card64::default()));
            }
        });
    });

    group.bench_function("omaha 4+5 cards", |b| {
        b.iter(|| {
            for &(p, bd) in &omaha {
                black_box(Game::Omaha.eval_rating(black_box(p), black_box(bd)));
            }
        });
    });

    group.bench_function("omaha5 5+5 cards", |b| {
        b.iter(|| {
            for &(p, bd) in &omaha5 {
                black_box(Game::Omaha5.eval_rating(black_box(p), black_box(bd)));
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
