#[allow(dead_code)]
mod common;

use common::*;

#[test]
const fn test_nuthiouts() {
    // TODO: impl this
}

#[test]
fn test_outstohandtype() {
    assert_int(
        "select avg(outstohandtype(hero, flop, straight)) from hero='TsJh', board='2s8h9d'",
        8,
    );

    assert_int(
        "select avg(outstohandtype(hero, turn, straight)) from hero='9dTd', board='2s3h7d8d'",
        6,
    );

    assert_int(
        "select avg(outstohandtype(hero, turn, straight)) from hero='9dTd', board='2d3h7d8d'",
        0,
    );

    assert_int(
        "select avg(outstohandtype(hero, flop, trips)) from hero='2s2h', board='2dQhAd'",
        4,
    );
}

#[test]
fn test_minoutstohandtype() {
    assert_yes(
        "select count(minoutstohandtype(hero, turn, straight, 4)) from hero='9sTh', board='JsQh2d'",
    );
}
