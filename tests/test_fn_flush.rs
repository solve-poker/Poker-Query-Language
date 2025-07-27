#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_threeflush() {
    assert_yes("select count(threeflush(hero, flop)) from game='holdem', board='sss', hero='sh'");
    assert_no("select count(threeflush(hero, flop)) from game='holdem', board='sss', hero='hh'");

    assert_yes("select count(threeflush(hero, flop)) from game='omaha', board='sss', hero='shhh'");
    assert_no("select count(threeflush(hero, flop)) from game='omaha', board='sss', hero='hhhh'");
}

#[test]
fn test_fourflush() {
    assert_yes("select count(fourflush(hero, turn)) from game='holdem', board='sssh', hero='sh'");
    assert_no("select count(fourflush(hero, turn)) from game='holdem', board='ssss', hero='hh'");

    assert_yes("select count(fourflush(hero, turn)) from game='omaha', board='sssh', hero='sshh'");
    assert_no("select count(fourflush(hero, turn)) from game='omaha', board='ssss', hero='hhhh'");
}
