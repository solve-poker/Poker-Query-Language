use crate::common::{assert_count_all, assert_count_none};

#[test]
fn royal_flush_on_board_is_nut_for_all() {
    assert_count_all(
        "select count(nuthi(hero, river)) \
         from game='holdem', hero='2c3c', villain='4d5d', board='AsKsQsJsTs'",
    );
}

#[test]
fn quad_aces_on_flop_is_nut() {
    assert_count_all(
        "select count(nuthi(hero, flop)) \
         from game='holdem', hero='AsKh', villain='2c3d', board='AhAdAc'",
    );
}

#[test]
fn nut_flush_on_turn_is_nut() {
    assert_count_all(
        "select count(nuthi(hero, turn)) \
         from game='holdem', hero='AsKs', villain='2c3c', board='QsJs5s2h'",
    );
}

#[test]
fn ace_high_is_not_nut_when_pair_possible() {
    assert_count_none(
        "select count(nuthi(hero, river)) \
         from game='holdem', hero='5d6d', villain='2c3c', board='AhKd7c2s9h'",
    );
}

#[test]
fn non_nut_flush_is_not_nut() {
    assert_count_none(
        "select count(nuthi(hero, river)) \
         from game='holdem', hero='2s3s', villain='7h8h', board='KsQsJs4c5d'",
    );
}
