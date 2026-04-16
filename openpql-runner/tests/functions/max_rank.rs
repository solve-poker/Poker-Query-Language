use crate::common::{assert_count_all, assert_count_none};

#[test]
fn max_rank_of_ace_high_flop_is_ace() {
    assert_count_all(
        "select count(maxrank(boardranks(flop)) = torank('A')) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn max_rank_of_low_flop_is_seven() {
    assert_count_all(
        "select count(maxrank(boardranks(flop)) = torank('7')) \
         from game='holdem', board='7h5d3c'",
    );
}

#[test]
fn max_rank_of_ace_high_river_is_ace() {
    assert_count_all(
        "select count(maxrank(boardranks(river)) = torank('A')) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}

#[test]
fn max_rank_of_flop_is_not_second_card() {
    assert_count_none(
        "select count(maxrank(boardranks(flop)) = torank('K')) \
         from game='holdem', board='AhKdQc'",
    );
}
