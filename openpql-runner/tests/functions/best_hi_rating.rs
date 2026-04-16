use crate::common::{assert_count_all, assert_count_none};

#[test]
fn outright_winner_is_best() {
    assert_count_all(
        "select count(besthirating(hero, river)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn loser_is_not_best() {
    assert_count_none(
        "select count(besthirating(villain, river)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn tied_hero_is_still_best() {
    assert_count_all(
        "select count(besthirating(hero, river)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn tied_villain_is_still_best() {
    assert_count_all(
        "select count(besthirating(villain, river)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn winner_on_flop_street_is_best() {
    assert_count_all(
        "select count(besthirating(hero, flop)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s'",
    );
}
