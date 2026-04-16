use crate::common::{assert_count_all, assert_count_none};

#[test]
fn top_pair_top_kicker_is_nut_within_pair() {
    assert_count_all(
        "select count(nuthiforhandtype(hero, flop)) \
         from game='holdem', hero='AsKh', villain='2c3d', board='AdTd2d'",
    );
}

#[test]
fn low_pair_is_not_nut_within_pair() {
    assert_count_none(
        "select count(nuthiforhandtype(hero, flop)) \
         from game='holdem', hero='3h3s', villain='4c5c', board='AdTd2d'",
    );
}

#[test]
fn pair_of_kings_not_nut_when_ace_on_board() {
    assert_count_none(
        "select count(nuthiforhandtype(hero, river)) \
         from game='holdem', hero='KhKs', villain='2c3c', board='Ad4h7s8d9h'",
    );
}

#[test]
fn royal_flush_is_nut_within_straight_flush() {
    assert_count_all(
        "select count(nuthiforhandtype(hero, river)) \
         from game='holdem', hero='2c3c', villain='4d5d', board='AsKsQsJsTs'",
    );
}

#[test]
fn quads_with_top_kicker_is_nut_within_quads() {
    assert_count_all(
        "select count(nuthiforhandtype(hero, flop)) \
         from game='holdem', hero='AsKh', villain='2c3d', board='AhAdAc'",
    );
}
