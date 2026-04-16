use crate::common::{assert_count_all, assert_count_none};

#[test]
fn aces_above_low_flop_is_overpair() {
    assert_count_all(
        "select count(overpair(hero, flop)) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn aces_above_low_turn_is_overpair() {
    assert_count_all(
        "select count(overpair(hero, turn)) \
         from game='holdem', hero='AhAc', board='2c3d4s5h'",
    );
}

#[test]
fn twos_below_top_card_is_not_overpair() {
    assert_count_none(
        "select count(overpair(hero, flop)) \
         from game='holdem', hero='2h2c', board='AcKdQs'",
    );
}

#[test]
fn pair_equal_to_top_card_is_not_overpair() {
    assert_count_none(
        "select count(overpair(hero, flop)) \
         from game='holdem', hero='QhQc', board='AcKdQs'",
    );
}

#[test]
fn non_paired_hand_is_not_overpair() {
    assert_count_none(
        "select count(overpair(hero, flop)) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}
