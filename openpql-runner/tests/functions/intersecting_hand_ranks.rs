use crate::common::{assert_count_all, assert_count_none};

#[test]
fn full_intersection_rank_count_matches_hand() {
    assert_count_all(
        "select count(rankcount(intersectinghandranks(hero, flop)) = 2) \
         from game='holdem', hero='AhKh', board='AcKdQs'",
    );
}

#[test]
fn no_intersection_gives_empty_set() {
    assert_count_all(
        "select count(rankcount(intersectinghandranks(hero, flop)) = 0) \
         from game='holdem', hero='AhKh', board='2c3d4s'",
    );
}

#[test]
fn partial_intersection_selects_matching_rank() {
    assert_count_all(
        "select count(maxrank(intersectinghandranks(hero, flop)) = torank('A')) \
         from game='holdem', hero='AhKh', board='AcJdQs'",
    );
}

#[test]
fn pocket_pair_on_board_contributes_single_rank() {
    assert_count_all(
        "select count(rankcount(intersectinghandranks(hero, flop)) = 1) \
         from game='holdem', hero='AhAc', board='AdKsQd'",
    );
}

#[test]
fn disjoint_board_has_no_nonzero_intersection() {
    assert_count_none(
        "select count(rankcount(intersectinghandranks(hero, flop)) > 0) \
         from game='holdem', hero='AhKh', board='2c3d4s'",
    );
}
