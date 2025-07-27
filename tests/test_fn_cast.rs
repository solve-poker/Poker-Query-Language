#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_tocard() {
    assert_yes(
        "select count(tocard('As') = tocard('aS')) from board='*', hero='*'",
    );
}

#[test]
fn test_torank() {
    assert_yes(
        "select count(torank('A') = torank('a')) from board='*', hero='*'",
    );
}
