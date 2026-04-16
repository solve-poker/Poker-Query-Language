use crate::common::{assert_count_all, assert_count_none};

#[test]
fn first_rank_of_flop_is_top() {
    assert_count_all(
        "select count(nthrank(1, boardranks(flop)) = torank('A')) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn second_rank_of_flop() {
    assert_count_all(
        "select count(nthrank(2, boardranks(flop)) = torank('K')) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn third_rank_of_flop() {
    assert_count_all(
        "select count(nthrank(3, boardranks(flop)) = torank('Q')) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn second_rank_of_paired_flop_skips_duplicates() {
    // Flop AhAdKc → rank set {A, K}, nth 1 = A, nth 2 = K.
    assert_count_all(
        "select count(nthrank(2, boardranks(flop)) = torank('K')) \
         from game='holdem', board='AhAdKc'",
    );
}

#[test]
fn second_rank_of_flop_is_not_top() {
    assert_count_none(
        "select count(nthrank(2, boardranks(flop)) = torank('A')) \
         from game='holdem', board='AhKdQc'",
    );
}
