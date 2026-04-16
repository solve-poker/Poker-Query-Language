use crate::common::{assert_count_all, assert_count_none};

#[test]
fn three_rank_flop_has_rankcount_three() {
    assert_count_all(
        "select count(rankcount(boardranks(flop)) = 3) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn paired_turn_has_rankcount_three() {
    // Turn AhAdKc2s → ranks = {A, K, 2} → 3 distinct.
    assert_count_all(
        "select count(rankcount(boardranks(turn)) = 3) \
         from game='holdem', board='AhAdKc2s'",
    );
}

#[test]
fn five_distinct_river_has_rankcount_five() {
    assert_count_all(
        "select count(rankcount(boardranks(river)) = 5) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}

#[test]
fn three_rank_flop_does_not_have_rankcount_two() {
    assert_count_none(
        "select count(rankcount(boardranks(flop)) = 2) \
         from game='holdem', board='AhKdQc'",
    );
}
