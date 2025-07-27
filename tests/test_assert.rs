mod common;

use common::*;

#[test]
#[should_panic(expected = " 1")]
fn test_int_err() {
    assert_int("select avg(1) from board='*'", 0);
}

#[test]
#[should_panic(expected = "100%")]
fn test_yes_err() {
    assert_yes("select count(1=0) from board='*'");
}

#[test]
#[should_panic(expected = "\\s0%")]
fn test_no_err() {
    assert_no("select count(1=1) from board='*'");
}

#[test]
#[should_panic(expected = "100%")]
fn test_err() {
    assert_err("select count(1=1) from board='*'", "100%");
}
