use crate::common::{assert_count_all, assert_count_none};

#[test]
fn pocket_aces_on_king_high_flop_is_trips() {
    assert_count_all(
        "select count(handtype(hero, flop) = trips) \
         from game='holdem', hero='AcAs', board='AhKdQc'",
    );
}

#[test]
fn suited_broadway_on_flop_is_straight_flush() {
    assert_count_all(
        "select count(handtype(hero, flop) = straightflush) \
         from game='holdem', hero='AhKh', board='QhJhTh'",
    );
}

#[test]
fn aces_full_of_kings_on_flop_is_full_house() {
    assert_count_all(
        "select count(handtype(hero, flop) = fullhouse) \
         from game='holdem', hero='AhAc', board='AsKhKd'",
    );
}

#[test]
fn heart_flush_on_river_is_flush() {
    assert_count_all(
        "select count(handtype(hero, river) = flush) \
         from game='holdem', hero='AhKh', board='QhJh2h3s5c'",
    );
}

#[test]
fn ace_high_miss_is_not_pair() {
    assert_count_none(
        "select count(handtype(hero, flop) = pair) \
         from game='holdem', hero='Ah2d', board='Kc9s4h'",
    );
}
