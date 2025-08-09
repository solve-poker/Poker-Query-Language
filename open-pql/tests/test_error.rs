#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_tocard_err() {
    assert_err(
        "select count(tocard('2As') = tocard('2As')) from board='*', hero='*'",
        "RuntimeError",
    );
}

#[test]
fn test_torank_err() {
    assert_err(
        "select count(torank('s') = torank('s')) from board='*', hero='*'",
        "RuntimeError",
    );
}

#[test]
fn test_pql_err() {
    assert_err(
        "select avg(1) from game='Mahjong', board='*', hero='*'",
        "UnrecognizedGame",
    );

    assert_err(
        "select avg(invalid(1)) from board='*', hero='*'",
        "UnrecognizedFunction",
    );

    assert_err(
        "select count(abc = efg) from board='*', hero='*'",
        "(?s)UnknownIdent.*efg",
    );

    assert_err(
        "select count(nthrank(256, boardranks(flop)) = torank('2')) from board='*', hero='*'",
        "ParseIntError.*too large",
    );

    assert_err("select invalid(1) from board='*', hero='*'", "InvalidPQL");

    assert_err(
        "select count(turncard(1) = tocard('2s')) from board='*', hero='*'",
        "WrongNumberOfArguments.*expected 0 got 1",
    );

    assert_err(
        "select count(boardranks(1) = tocard('2s')) from board='*', hero='*'",
        "TypeError.*expected Street",
    );

    assert_err(
        "select avg(1) from board='*', hero='AAAA'",
        "(?s)InvalidRange.*AAAA",
    );
}
