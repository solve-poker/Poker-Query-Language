use crate::common::{assert_count_all, assert_count_none};

#[test]
fn no_overlap_flop_has_zero_intersections() {
    assert_count_all(
        "select count(handboardintersections(hero, flop) = 0) \
         from game='holdem', hero='AhKh', board='2c3d4s'",
    );
}

#[test]
fn both_hole_cards_share_rank_with_flop_gives_two() {
    assert_count_all(
        "select count(handboardintersections(hero, flop) = 2) \
         from game='holdem', hero='AhKh', board='AcKdQs'",
    );
}

#[test]
fn one_hole_card_shares_rank_with_flop_gives_one() {
    assert_count_all(
        "select count(handboardintersections(hero, flop) = 1) \
         from game='holdem', hero='AhKh', board='AcJdQs'",
    );
}

#[test]
fn river_with_full_overlap_still_gives_hand_rank_count() {
    assert_count_all(
        "select count(handboardintersections(hero, river) = 2) \
         from game='holdem', hero='AhKh', board='AcKdQsJs2s'",
    );
}

#[test]
fn pocket_pair_hitting_board_intersects_as_one_rank() {
    assert_count_all(
        "select count(handboardintersections(hero, flop) = 1) \
         from game='holdem', hero='AhAc', board='AdKsQs'",
    );
}

#[test]
fn board_with_no_overlap_is_not_nonzero() {
    assert_count_none(
        "select count(handboardintersections(hero, flop) > 0) \
         from game='holdem', hero='AhKh', board='2c3d4s'",
    );
}
