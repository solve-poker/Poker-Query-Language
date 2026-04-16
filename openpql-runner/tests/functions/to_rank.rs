use crate::common::{assert_count_all, assert_count_none};

#[test]
fn parses_rank_equal_to_itself() {
    assert_count_all(
        "select count(torank('A') = torank('A')) \
         from game='holdem', board='*'",
    );
}

#[test]
fn different_ranks_are_not_equal() {
    assert_count_none(
        "select count(torank('A') = torank('K')) \
         from game='holdem', board='*'",
    );
}

#[test]
fn ace_is_greater_than_king() {
    assert_count_all(
        "select count(torank('A') > torank('K')) \
         from game='holdem', board='*'",
    );
}

#[test]
fn two_is_less_than_three() {
    assert_count_all(
        "select count(torank('2') < torank('3')) \
         from game='holdem', board='*'",
    );
}
