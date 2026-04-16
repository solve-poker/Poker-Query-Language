use crate::common::{assert_count_all, assert_count_none};

#[test]
fn holdem_pocket_pair_is_pair() {
    assert_count_all(
        "select count(pocketpair(hero)) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn holdem_non_pair_is_not_pair() {
    assert_count_none(
        "select count(pocketpair(hero)) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}

#[test]
fn omaha_hand_with_pair_is_pair() {
    assert_count_all(
        "select count(pocketpair(hero)) \
         from game='omaha', hero='AhAcKsQd', board='2c3d4s'",
    );
}

#[test]
fn omaha_hand_without_pair_is_not_pair() {
    assert_count_none(
        "select count(pocketpair(hero)) \
         from game='omaha', hero='AhKcQsJd', board='2c3d4s'",
    );
}
