use crate::common::{assert_count_all, assert_count_none};

#[test]
fn disjoint_board_gives_all_hand_ranks() {
    assert_count_all(
        "select count(rankcount(nonintersectinghandranks(hero, flop)) = 2) \
         from game='holdem', hero='AhKh', board='2c3d4s'",
    );
}

#[test]
fn fully_overlapping_board_gives_empty_set() {
    assert_count_all(
        "select count(rankcount(nonintersectinghandranks(hero, flop)) = 0) \
         from game='holdem', hero='AhKh', board='AcKdQs'",
    );
}

#[test]
fn partial_overlap_leaves_unique_hand_rank() {
    assert_count_all(
        "select count(maxrank(nonintersectinghandranks(hero, flop)) = torank('K')) \
         from game='holdem', hero='AhKh', board='AcJdQs'",
    );
}

#[test]
fn pocket_pair_with_rank_on_board_has_empty_nonintersection() {
    assert_count_all(
        "select count(rankcount(nonintersectinghandranks(hero, flop)) = 0) \
         from game='holdem', hero='AhAc', board='AdKsQd'",
    );
}

#[test]
fn fully_overlapping_board_is_never_nonzero() {
    assert_count_none(
        "select count(rankcount(nonintersectinghandranks(hero, flop)) > 0) \
         from game='holdem', hero='AhKh', board='AcKdQs'",
    );
}
