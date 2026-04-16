use crate::common::{assert_count_all, assert_count_none};

#[test]
fn wildcard_range_matches_any_board() {
    assert_count_all(
        "select count(boardinrange('*')) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}

#[test]
fn exact_matching_range_always_matches() {
    assert_count_all(
        "select count(boardinrange('AhKdQc2s3h')) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}

#[test]
fn paired_aces_range_matches_paired_flop() {
    // Flop has two aces, so 'AA' (two aces in flop) matches every trial.
    assert_count_all(
        "select count(boardinrange('AA')) \
         from game='holdem', board='AhAdKc2s3h'",
    );
}

#[test]
fn paired_aces_range_does_not_match_unpaired_flop() {
    // Flop has only one ace, so 'AA' never matches.
    assert_count_none(
        "select count(boardinrange('AA')) \
         from game='holdem', board='AhKdQc2s3h'",
    );
}
