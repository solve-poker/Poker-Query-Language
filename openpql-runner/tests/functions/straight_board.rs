use crate::common::{assert_count_all, assert_count_none};

#[test]
fn consecutive_flop_is_straight() {
    assert_count_all(
        "select count(straightboard(flop)) \
         from game='holdem', board='5h6c7d'",
    );
}

#[test]
fn wheel_flop_is_straight() {
    // A-2-3 can complete a 5-high straight (wheel).
    assert_count_all(
        "select count(straightboard(flop)) \
         from game='holdem', board='Ah2c3d'",
    );
}

#[test]
fn broadway_flop_is_straight() {
    // Q-K-A can complete a broadway straight.
    assert_count_all(
        "select count(straightboard(flop)) \
         from game='holdem', board='QhKcAd'",
    );
}

#[test]
fn gapped_flop_is_not_straight() {
    assert_count_none(
        "select count(straightboard(flop)) \
         from game='holdem', board='2h5c9d'",
    );
}

#[test]
fn made_straight_river_is_straight() {
    assert_count_all(
        "select count(straightboard(river)) \
         from game='holdem', board='5h6c7d8s9h'",
    );
}

#[test]
fn shortdeck_ninth_to_jack_is_straight() {
    assert_count_all(
        "select count(straightboard(flop)) \
         from game='shortdeck', board='9hTcJd'",
    );
}
