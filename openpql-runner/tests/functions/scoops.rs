use crate::common::{assert_count_all, assert_count_none};

#[test]
fn outright_winner_scoops() {
    assert_count_all(
        "select count(scoops(hero)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn loser_does_not_scoop() {
    assert_count_none(
        "select count(scoops(villain)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn split_pot_hero_does_not_scoop() {
    assert_count_none(
        "select count(scoops(hero)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn split_pot_villain_does_not_scoop() {
    assert_count_none(
        "select count(scoops(villain)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn three_way_tie_no_one_scoops() {
    assert_count_none(
        "select count(scoops(v2)) \
         from game='holdem', hero='2c3c', villain='4d5d', v2='6h7h', board='AsKsQsJsTs'",
    );
}
