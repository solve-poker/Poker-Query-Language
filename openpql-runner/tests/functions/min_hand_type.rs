use crate::common::{assert_count_all, assert_count_none};

#[test]
fn trips_meets_min_pair() {
    assert_count_all(
        "select count(minhandtype(hero, flop, pair)) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn trips_meets_min_trips() {
    assert_count_all(
        "select count(minhandtype(hero, flop, trips)) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn trips_does_not_meet_min_quads() {
    assert_count_none(
        "select count(minhandtype(hero, flop, quads)) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn high_card_does_not_meet_min_pair() {
    assert_count_none(
        "select count(minhandtype(hero, flop, pair)) \
         from game='holdem', hero='Ah2d', board='Kc9s4h'",
    );
}

#[test]
fn straight_flush_meets_min_flush() {
    assert_count_all(
        "select count(minhandtype(hero, flop, flush)) \
         from game='holdem', hero='AhKh', board='QhJhTh'",
    );
}
