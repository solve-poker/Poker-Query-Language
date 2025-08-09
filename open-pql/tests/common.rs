use std::io;

use open_pql::StatementsRunner;
use regex::Regex;

#[allow(clippy::missing_panics_doc)]
pub fn run(s: &str) -> (String, String) {
    fn to_s(b: Box<dyn io::Write>) -> String {
        let ptr: *mut Vec<u8> = Box::into_raw(b).cast();
        unsafe { String::from_utf8(*Box::from_raw(ptr)).unwrap() }
    }

    const N_TRIALS: usize = 1;
    const N_THREADS: usize = 1;

    let mut r = StatementsRunner::new(
        s,
        N_TRIALS,
        N_THREADS,
        Box::new(Vec::<u8>::new()),
        Box::new(Vec::<u8>::new()),
    );

    r.run();

    let StatementsRunner {
        stream_out,
        stream_err,
        ..
    } = r;

    (to_s(stream_out), to_s(stream_err))
}

#[allow(clippy::missing_panics_doc)]
pub fn assert_match(s: &str, re: &str) {
    let (out, _) = run(s);
    let re = Regex::new(re).unwrap();

    assert!(re.is_match(&out), "{out:?}\n{re:?}");
}

#[allow(clippy::missing_panics_doc)]
pub fn assert_err(s: &str, re: &str) {
    let (_, err) = run(s);
    let re = Regex::new(re).unwrap();

    assert!(re.is_match(&err), "{err:?}\n{re:?}");
}

pub fn assert_int(s: &str, i: isize) {
    assert_match(s, &format!(r"(?m){i}$"));
}

pub fn assert_yes(s: &str) {
    assert_match(s, r"100%");
}

pub fn assert_no(s: &str) {
    assert_match(s, r"\s0%");
}
