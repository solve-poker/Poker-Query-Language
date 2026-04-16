use crate::common::{assert_count_all, assert_count_none};

#[test]
fn three_distinct_ranks_on_flop() {
    assert_count_all(
        "select count(rankcount(boardranks(flop)) = 3) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn paired_flop_has_two_distinct_ranks() {
    assert_count_all(
        "select count(rankcount(boardranks(flop)) = 2) \
         from game='holdem', board='AhAdKc'",
    );
}

#[test]
fn five_distinct_ranks_on_river() {
    assert_count_all(
        "select count(rankcount(boardranks(river)) = 5) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}

#[test]
fn paired_flop_does_not_have_three_ranks() {
    assert_count_none(
        "select count(rankcount(boardranks(flop)) = 3) \
         from game='holdem', board='AhAdKc'",
    );
}
