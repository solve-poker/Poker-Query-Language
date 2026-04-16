use crate::common::{assert_count_all, assert_count_none};

#[test]
fn outright_winner_does_not_tie() {
    assert_count_none(
        "select count(tieshi(hero)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn loser_does_not_tie() {
    assert_count_none(
        "select count(tieshi(villain)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn split_pot_hero_ties() {
    assert_count_all(
        "select count(tieshi(hero)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn split_pot_villain_ties() {
    assert_count_all(
        "select count(tieshi(villain)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn three_way_royal_flush_all_tie() {
    assert_count_all(
        "select count(tieshi(hero)) \
         from game='holdem', hero='2c3c', villain='4d5d', v2='6h7h', board='AsKsQsJsTs'",
    );
}
