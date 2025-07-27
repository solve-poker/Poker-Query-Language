#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_handtype() {
    let results = [
        "STRAIGHT_FLUSH",
        "QUADS",
        "FULL_HOUSE",
        "FLUSH",
        "STRAIGHT",
        "TRIPS",
        "TWO_PAIR",
        "PAIR",
        "HIGH_CARD",
    ];

    assert_match(
        "
        select max(handtype(hero, flop)) from hero='8s9s', board='7s6sTs';
        select max(handtype(hero, flop)) from hero='8s8h', board='8d8cTs';
        select max(handtype(hero, flop)) from hero='7s6s', board='7h6h6c';
        select max(handtype(hero, flop)) from hero='8s9s', board='7s6s2s';
        select max(handtype(hero, flop)) from hero='8d9s', board='7s6hTc';
        select max(handtype(hero, flop)) from hero='7d8s', board='7s7hTc';
        select max(handtype(hero, flop)) from hero='7c8c', board='7s8hTc';
        select max(handtype(hero, flop)) from hero='4s4h', board='6s8hTc';
        select max(handtype(hero, flop)) from hero='4s2h', board='6s8hTc'
         ",
        &format!("(?s){}", results.join(".*")),
    );
}

#[test]
fn test_exacthandtype() {
    assert_yes(
        "select count(exacthandtype(hero, flop, pair)) from board='AKQ', hero='AT'",
    );
}

#[test]
fn test_minhandtype() {
    assert_yes(
        "select count(minhandtype(hero, flop, highcard)) from board='AKQ', hero='AT'",
    );

    assert_no(
        "select count(minhandtype(hero, flop, flush)) from game='shortdeck', board='KKK', hero='AA'",
    );
}

#[test]
fn test_hirating() {
    assert_yes(
        "select count(hirating(p1, flop) > hirating(p2, flop)) from board='AKQ', p1='AT', p2='KT'",
    );

    assert_no(
        "select count(hirating(p1, river) > hirating(p2, river)) from game='shortdeck', board='AAAsKsQs', p1='TT', p2='7s8s'",
    );
}

#[test]
fn test_minhirating() {
    assert_no(
        "select count(minhirating(hero, flop, ratehihand('AsAhAdKsKh'))) from board='AKQ', hero='AT'",
    );
}

#[test]
fn test_ratehihand() {
    assert_yes(
        "select count(ratehihand('2h3hThJhQh') = ratehihand('2s3sTsJsQs')) from board='*'",
    );
}

#[test]
fn test_fivecardhihandnumber() {
    assert_yes(
        "select count(fivecardhihandnumber(hero, flop) = 1) from board='AdKdQd', hero='JdTd'",
    );
}

#[test]
fn test_nuthi() {
    assert_no(
        "select count(nuthi(hero, flop)) from board='TsJsQs', hero='8s9s'",
    );

    assert_yes(
        "select count(nuthi(hero, flop)) from game='omaha', board='TsJsQs', hero='8s9sKs2h'",
    );
}

#[test]
fn test_nuthiforhandtype() {
    assert_no(
        "select count(nuthiforhandtype(hero, flop)) from board='TJQ', hero='AT'",
    );
}
