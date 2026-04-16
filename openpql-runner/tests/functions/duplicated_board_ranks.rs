use crate::common::{assert_count_all, assert_count_none};

#[test]
fn unpaired_flop_has_no_duplicates() {
    assert_count_all(
        "select count(rankcount(duplicatedboardranks(flop)) = 0) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn paired_flop_has_one_duplicate_rank() {
    assert_count_all(
        "select count(rankcount(duplicatedboardranks(flop)) = 1) \
         from game='holdem', board='AhAdKc'",
    );
}

#[test]
fn two_pair_river_has_two_duplicate_ranks() {
    assert_count_all(
        "select count(rankcount(duplicatedboardranks(river)) = 2) \
         from game='holdem', board='AhAdKsKc2h'",
    );
}

#[test]
fn unpaired_flop_does_not_have_one_duplicate() {
    assert_count_none(
        "select count(rankcount(duplicatedboardranks(flop)) = 1) \
         from game='holdem', board='AhKdQc'",
    );
}
