#[allow(dead_code)]
mod common;

use common::*;

#[test]
fn test_flophandcategory() {
    let results = [
        "FLOPSTRAIGHTFLUSH",
        "FLOPQUADS",
        "FLOPFULLHOUSE",
        "FLOPFLUSH",
        "FLOPSTRAIGHT",
        "FLOPSET",
        "FLOPTRIPS",
        "FLOPTOPTWO",
        "FLOPTOPANDBOTTOM",
        "FLOPBOTTOMTWO",
        "FLOPOVERPAIR",
        "FLOPTOPPAIR",
        "FLOPPOCKET12",
        "FLOPSECONDPAIR",
        "FLOPPOCKET23",
        "FLOPTHIRDPAIR",
        "FLOPUNDERPAIR",
        "FLOPNOTHING",
    ];

    assert_match(
        "
         select max(flophandcategory(hero)) from hero='8s9s', board='7s6sts';
         select max(flophandcategory(hero)) from hero='8s8h', board='8d8cts';
         select max(flophandcategory(hero)) from hero='7s6s', board='7h6h6c';
         select max(flophandcategory(hero)) from hero='8s9s', board='7s6s2s';
         select max(flophandcategory(hero)) from hero='8d9s', board='7s6htc';
         select max(flophandcategory(hero)) from hero='7h7d', board='7s6htc';
         select max(flophandcategory(hero)) from hero='7d8s', board='7s7htc';
         select max(flophandcategory(hero)) from hero='8sts', board='7s8htc';
         select max(flophandcategory(hero)) from hero='7cts', board='7s8htc';
         select max(flophandcategory(hero)) from hero='7c8c', board='7s8htc';
         select max(flophandcategory(hero)) from hero='jsjh', board='7s8htc';
         select max(flophandcategory(hero)) from hero='tsah', board='7s8htc';
         select max(flophandcategory(hero)) from hero='9s9h', board='7s8htc';
         select max(flophandcategory(hero)) from hero='8sah', board='7s8htc';
         select max(flophandcategory(hero)) from hero='7s7h', board='6s8htc';
         select max(flophandcategory(hero)) from hero='7hah', board='7s8htc';
         select max(flophandcategory(hero)) from hero='4s4h', board='6s8htc';
         select max(flophandcategory(hero)) from hero='4s2h', board='6s8htc';
         ",
        &format!("(?s){}", results.join(".*")),
    );
}

#[test]
fn test_exactflophandcategory() {
    assert_yes(
        "select count(exactflophandcategory(hero, floptoppair)) from board='AKQ', hero='AT'",
    );
}

#[test]
fn test_minflophandcategory() {
    assert_yes(
        "select count(minflophandcategory(hero, flopfullhouse)) from game='shortdeck', board='7s8s9s', hero='AsKs'",
    );
}
