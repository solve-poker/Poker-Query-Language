use crate::common::{assert_count_all, assert_count_none};

#[test]
fn holdem_two_distinct_ranks_count_is_two() {
    assert_count_all(
        "select count(rankcount(handranks(hero)) = 2) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}

#[test]
fn holdem_pocket_pair_rank_count_is_one() {
    assert_count_all(
        "select count(rankcount(handranks(hero)) = 1) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn omaha_four_distinct_ranks_count_is_four() {
    assert_count_all(
        "select count(rankcount(handranks(hero)) = 4) \
         from game='omaha', hero='AhKhQhJh', board='2c3d4s'",
    );
}

#[test]
fn omaha_two_pair_rank_count_is_two() {
    assert_count_all(
        "select count(rankcount(handranks(hero)) = 2) \
         from game='omaha', hero='AhAcKsKd', board='2c3d4s'",
    );
}

#[test]
fn holdem_max_hand_rank_is_ace() {
    assert_count_all(
        "select count(maxrank(handranks(hero)) = torank('A')) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}

#[test]
fn holdem_low_hand_max_rank_is_not_ace() {
    assert_count_none(
        "select count(maxrank(handranks(hero)) = torank('A')) \
         from game='holdem', hero='2h3c', board='AcKdQs'",
    );
}
