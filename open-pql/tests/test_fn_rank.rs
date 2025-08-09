#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_boardranks() {
    assert_int(
        "select avg(rankcount(boardranks(flop))) from board='AKQJT'",
        3,
    );
    assert_int(
        "select avg(rankcount(boardranks(turn))) from board='AKQJT'",
        4,
    );
    assert_int(
        "select avg(rankcount(boardranks(river))) from board='AKQJT'",
        5,
    );
}

#[test]
fn test_duplicatedboardranks() {
    assert_int(
        "select avg(rankcount(duplicatedboardranks(flop))) from board='AAK'",
        1,
    );
}

#[test]
fn test_handranks() {
    assert_int(
        "select avg(rankcount(handranks(hero, flop))) from board='*', hero='AA'",
        1,
    );
}

#[test]
fn test_duplicatedhandranks() {
    assert_int(
        "select avg(rankcount(duplicatedhandranks(hero, flop))) from board='*', hero='AA'",
        1,
    );
}

#[test]
fn test_intersectinghandranks() {
    assert_int(
        "select avg(rankcount(intersectinghandranks(hero, flop))) from board='AKK', hero='AA'",
        1,
    );
}

#[test]
fn test_nonintersectinghandranks() {
    assert_int(
        "select avg(rankcount(nonintersectinghandranks(hero, flop))) from board='AKK', hero='AK'",
        0,
    );
}

#[test]
fn test_handboardintersections() {
    assert_int(
        "select avg(handboardintersections(hero, flop)) from board='AKK', hero='AA'",
        1,
    );
}

#[test]
fn test_hastopboardrank() {
    assert_yes(
        "select count(hastopboardrank(hero, flop)) from board='AKK', hero='AA'",
    );
}

#[test]
fn test_hassecondboardrank() {
    assert_yes(
        "select count(hassecondboardrank(hero, flop)) from board='AKK', hero='K2'",
    );
}

#[test]
fn test_pocketpair() {
    assert_yes(
        "select count(pocketpair(hero)) from game='omaha', board='*', hero='AA34'",
    );
}

#[test]
fn test_overpair() {
    assert_yes(
        "select count(overpair(hero, flop)) from game='omaha', board='789', hero='22TT'",
    );
}

#[test]
fn test_minrank() {
    assert_yes(
        "select count(minrank(boardranks(flop)) = toRank('7')) from board='789'",
    );
}

#[test]
fn test_maxrank() {
    assert_yes(
        "select count(maxrank(boardranks(flop)) = toRank('9')) from board='789'",
    );
}

#[test]
fn test_nthrank() {
    assert_yes(
        "select count(nthrank(1, boardranks(flop)) = toRank('9')) from board='789'",
    );
    assert_yes(
        "select count(nthrank(2, boardranks(flop)) = toRank('8')) from board='789'",
    );
    assert_yes(
        "select count(nthrank(3, boardranks(flop)) = toRank('7')) from board='789'",
    );

    assert_no(
        "select count(nthrank(2, boardranks(flop)) = toRank('7')) from board='AAA'",
    );
}
