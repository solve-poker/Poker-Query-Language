#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_inrange() {
    assert_yes("select count(inrange(hero, 'RR')) from board='*', hero='AA'");
}

#[test]
fn test_boardinrange() {
    assert_no("select count(boardinrange('***AK')) from board='222KA'");
    assert_yes("select count(boardinrange('AJQ')) from board='JQA'");
}
