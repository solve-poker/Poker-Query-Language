use crate::common::{assert_count_all, assert_count_none};

#[test]
fn trips_matches_exact_trips() {
    assert_count_all(
        "select count(exacthandtype(hero, flop, trips)) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn trips_does_not_match_exact_pair() {
    assert_count_none(
        "select count(exacthandtype(hero, flop, pair)) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn quads_on_flop_matches_exact_quads() {
    assert_count_all(
        "select count(exacthandtype(hero, flop, quads)) \
         from game='holdem', hero='AhAc', board='AsAd2c'",
    );
}

#[test]
fn river_flush_matches_exact_flush() {
    assert_count_all(
        "select count(exacthandtype(hero, river, flush)) \
         from game='holdem', hero='AhKh', board='QhJh2h3s5c'",
    );
}

#[test]
fn high_card_does_not_match_exact_straight() {
    assert_count_none(
        "select count(exacthandtype(hero, flop, straight)) \
         from game='holdem', hero='Ah2d', board='Kc9s4h'",
    );
}
