extern crate fn_macro;

use std::io;

use open_pql::StatementsRunner;

#[cfg(not(debug_assertions))]
const N: usize = 600_000;

#[cfg(not(debug_assertions))]
const T: usize = 8;

#[cfg(debug_assertions)]
const N: usize = 6000;

#[cfg(debug_assertions)]
const T: usize = 1;

fn main() {
    //let src = "select avg(boardsuitcount(river)) from hero='As9s', villain='*', board='2s3sJh', game='holdem'";
    //let src = "select max(RateHiHand('AsQsKsTsJs')) from game='holdem', hero='*', board='*'";
    //let src = "select count(maxrank(boardranks(flop)) = torank('k')) from game='holdem', hero='*', board='*'";
    //let src = "select count(turncard() = tocard('ks')) from game='holdem', hero='*', board='*'";
    //let src = "
    //select avg(boardsuitcount(flop)) from game='holdem', hero='*', board='*';
    //select count(turncard() = tocard('ks')) from game='holdem', hero='*', board='*';
    //select max(RateHiHand('AsQsKsTsJs')) from game='holdem', hero='*', board='*';
    //select min(hiRating(hero, river)) from game='holdem', hero='*', board='*';
    //";
    let src = "select avg(equity(hero, turn)) from hero='A', villain='*', board='*', game='holdem'";

    let mut r = StatementsRunner::new(
        src,
        N,
        T,
        Box::new(Vec::<u8>::new()),
        Box::new(Vec::<u8>::new()),
    );

    //dbg!(std::mem::size_of::<open_pql::vm::VmOpValue>() * 8);

    r.run();

    let StatementsRunner {
        stream_out,
        stream_err,
        ..
    } = r;

    println!("{}", to_s(stream_out));
    println!("{}", to_s(stream_err));
}

fn to_s(b: Box<dyn io::Write>) -> String {
    let ptr: *mut Vec<u8> = Box::into_raw(b).cast();
    unsafe { String::from_utf8(*Box::from_raw(ptr)).unwrap() }
}
