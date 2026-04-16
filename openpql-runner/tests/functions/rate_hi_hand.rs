use crate::common::{assert_count_all, assert_count_none};

#[test]
fn royal_flush_beats_high_card() {
    assert_count_all(
        "select count(ratehihand('AsKsQsJsTs') > ratehihand('2h3c4d5s7c')) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn same_hand_has_equal_rating() {
    assert_count_all(
        "select count(ratehihand('AsKsQsJsTs') = ratehihand('AsKsQsJsTs')) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn royal_flushes_of_different_suits_tie() {
    assert_count_all(
        "select count(ratehihand('AsKsQsJsTs') = ratehihand('AhKhQhJhTh')) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn quads_beat_flush() {
    assert_count_all(
        "select count(ratehihand('AsAhAdAcKs') > ratehihand('AsKsQsJs9s')) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn high_card_does_not_beat_full_house() {
    assert_count_none(
        "select count(ratehihand('2h3c4d5s7c') > ratehihand('AsAhAdKsKh')) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}
