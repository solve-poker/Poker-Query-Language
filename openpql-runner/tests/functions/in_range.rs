use crate::common::{assert_count_all, assert_count_none};

#[test]
fn aces_in_aa_range() {
    assert_count_all(
        "select count(inrange(hero, 'AA')) \
         from game='holdem', hero='AhAc', board='2c3d4s'",
    );
}

#[test]
fn non_pair_not_in_aa_range() {
    assert_count_none(
        "select count(inrange(hero, 'AA')) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}

#[test]
fn ak_in_broadway_or_range() {
    assert_count_all(
        "select count(inrange(hero, 'AA, AK')) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}

#[test]
fn offsuit_ak_not_in_aks_range() {
    assert_count_none(
        "select count(inrange(hero, 'AKs')) \
         from game='holdem', hero='AhKc', board='2c3d4s'",
    );
}

#[test]
fn pair_in_pair_plus_range() {
    assert_count_all(
        "select count(inrange(hero, 'TT+')) \
         from game='holdem', hero='QhQc', board='2c3d4s'",
    );
}
