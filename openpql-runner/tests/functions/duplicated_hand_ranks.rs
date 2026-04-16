use crate::common::{assert_count_all, assert_count_none};

#[test]
fn holdem_pocket_pair_has_one_duplicated_rank() {
    assert_count_all(
        "select count(rankcount(duplicatedhandranks(hero)) = 1) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn holdem_non_pair_has_no_duplicated_ranks() {
    assert_count_all(
        "select count(rankcount(duplicatedhandranks(hero)) = 0) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}

#[test]
fn omaha_two_pair_has_two_duplicated_ranks() {
    assert_count_all(
        "select count(rankcount(duplicatedhandranks(hero)) = 2) \
         from game='omaha', hero='AhAcKsKd', board='2c3d4s'",
    );
}

#[test]
fn omaha_single_pair_has_one_duplicated_rank() {
    assert_count_all(
        "select count(rankcount(duplicatedhandranks(hero)) = 1) \
         from game='omaha', hero='AhAcKsQd', board='2c3d4s'",
    );
}

#[test]
fn holdem_pocket_pair_duplicated_rank_is_the_pair_rank() {
    assert_count_all(
        "select count(maxrank(duplicatedhandranks(hero)) = torank('A')) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn holdem_non_pair_has_no_duplicated_rank_value() {
    assert_count_none(
        "select count(rankcount(duplicatedhandranks(hero)) > 0) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}
