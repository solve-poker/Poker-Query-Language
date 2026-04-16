use crate::common::{assert_count_all, assert_count_none};

#[test]
fn monotone_flop_has_one_suit() {
    assert_count_all(
        "select count(boardsuitcount(flop) = 1) \
         from game='holdem', board='AhKhQh'",
    );
}

#[test]
fn rainbow_flop_has_three_suits() {
    assert_count_all(
        "select count(boardsuitcount(flop) = 3) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn twotone_flop_has_two_suits() {
    assert_count_all(
        "select count(boardsuitcount(flop) = 2) \
         from game='holdem', board='AhKhQd'",
    );
}

#[test]
fn four_suit_river_has_four_suits() {
    // Ah Kd Qc 2s 3h → hearts, diamonds, clubs, spades.
    assert_count_all(
        "select count(boardsuitcount(river) = 4) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}

#[test]
fn monotone_flop_does_not_have_two_suits() {
    assert_count_none(
        "select count(boardsuitcount(flop) = 2) \
         from game='holdem', board='AhKhQh'",
    );
}
