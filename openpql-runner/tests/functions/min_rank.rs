use crate::common::{assert_count_all, assert_count_none};

#[test]
fn min_rank_of_broadway_flop_is_queen() {
    assert_count_all(
        "select count(minrank(boardranks(flop)) = torank('Q')) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn min_rank_of_low_flop_is_three() {
    assert_count_all(
        "select count(minrank(boardranks(flop)) = torank('3')) \
         from game='holdem', board='7h5d3c'",
    );
}

#[test]
fn min_rank_of_river_is_two() {
    assert_count_all(
        "select count(minrank(boardranks(river)) = torank('2')) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}

#[test]
fn min_rank_of_flop_is_not_top_card() {
    assert_count_none(
        "select count(minrank(boardranks(flop)) = torank('A')) \
         from game='holdem', board='AhKdQc'",
    );
}
