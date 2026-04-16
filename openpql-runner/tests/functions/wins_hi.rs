use crate::common::{assert_count_all, assert_count_none};

#[test]
fn outright_winner_wins_hi() {
    assert_count_all(
        "select count(winshi(hero)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn loser_does_not_win_hi() {
    assert_count_none(
        "select count(winshi(villain)) \
         from game='holdem', hero='AhAs', villain='KhKs', board='2c3d4s7h8c'",
    );
}

#[test]
fn tied_hero_still_wins_hi() {
    assert_count_all(
        "select count(winshi(hero)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn tied_villain_still_wins_hi() {
    assert_count_all(
        "select count(winshi(villain)) \
         from game='holdem', hero='AhAd', villain='AcAs', board='2c3d4s7h8c'",
    );
}

#[test]
fn three_way_royal_flush_everyone_wins() {
    assert_count_all(
        "select count(winshi(v2)) \
         from game='holdem', hero='2c3c', villain='4d5d', v2='6h7h', board='AsKsQsJsTs'",
    );
}
