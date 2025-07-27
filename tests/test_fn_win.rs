#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_winshi() {
    assert_yes(
        "select count(winshi(p2)) from board='2s3h4cJsKh', p1='AsAh', p2='KcKd'",
    );
}

#[test]
fn test_besthirating() {
    assert_no("select count(besthirating(p1, flop)) from board='AsKsQhJdTc', p1='AhKh', p2='JsTs'");
    assert_no("select count(besthirating(p1, turn)) from board='AsKsQhJdTc', p1='AhKh', p2='JsTs'");
    assert_yes("select count(besthirating(p1, river)) from board='AsKsQhJdTc', p1='AhKh', p2='JsTs'");
}

#[test]
fn test_winninghandtype() {
    assert_yes("select count(winninghandtype() = flush) from game='holdem', p1='KhKd', p2='5s6s', board='KsQsJs2c4d'");
}

#[test]
fn test_tieshi() {
    assert_yes("select count(tieshi(p1)) from game='holdem', p1='KdKc', p2='QdQc', board='As2s3s4s5s'");
    assert_no("select count(tieshi(p1)) from game='holdem', p1='KdKc', board='As2s3s4s5s'");
}

#[test]
fn test_scoops() {
    assert_no("select count(scoops(p1)) from game='holdem', p1='KdKc', p2='QdQc', board='As2s3s4s5s'");
    assert_yes("select count(scoops(p1)) from game='holdem', p1='KdKc', board='As2s3s4s5s'");
}
