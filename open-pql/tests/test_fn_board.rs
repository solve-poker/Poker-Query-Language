#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_boardsuitcount() {
    assert_int("select avg(boardsuitcount(flop)) from board='ssshd'", 1);
    assert_int("select avg(boardsuitcount(turn)) from board='ssshd'", 2);
    assert_int("select avg(boardsuitcount(river)) from board='ssshd'", 3);
}

#[test]
fn test_rainbowboard() {
    assert_yes("select count(rainbowboard(flop)) from board='shc'");
    assert_no("select count(rainbowboard(river)) from board='shdc'");
}

#[test]
fn test_monotoneboard() {
    assert_yes("select count(monotoneboard(flop)) from board='ssshd'");
    assert_no("select count(monotoneboard(turn)) from board='hhhss'");
}

#[test]
fn test_twotoneboard() {
    assert_no("select count(twotoneboard(flop)) from board='shc'");
    assert_yes("select count(twotoneboard(river)) from board='ssshh'");
}

#[test]
fn test_flushingboard() {
    assert_yes("select count(flushingboard(flop)) from board='ssshd'");
    assert_no("select count(flushingboard(turn)) from board='hhsss'");
}

#[test]
fn test_straightboard() {
    assert_yes(
        "select count(straightboard(flop)) from game='holdem', board='A23'",
    );
    assert_yes(
        "select count(straightboard(flop)) from game='shortdeck', board='A78'",
    );
    assert_no(
        "select count(straightboard(flop)) from game='omaha', board='A78'",
    );
}

#[test]
fn test_pairedboard() {
    assert_no("select count(pairedboard(flop)) from board='AKQJJ'");
    assert_yes("select count(pairedboard(river)) from board='AKQJJ'");
}

#[test]
fn test_turncard() {
    assert_yes("select count(turncard() = tocard('2s')) from board='AAA2s'");
    assert_no("select count(turncard() = tocard('2h')) from board='AAA2s'");
}

#[test]
fn test_rivercard() {
    assert_yes("select count(rivercard() = tocard('2s')) from board='AAAA2s'");
    assert_no("select count(rivercard() = tocard('2h')) from board='AAAA2s'");
}
