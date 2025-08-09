use std::{
    env::{self},
    fs,
    path::Path,
};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use speedy::Writable;

#[allow(dead_code)]
pub mod prim {
    include!("src/prim/mod.rs");
}

fn main() {
    build_lalrpop_range();
    build_lalrpop_pql();
    gen_holdem_abs_ranking_num();
    gen_shortdeck_abs_ranking_num();
}

fn build_lalrpop_range() {
    let source = "src/range_parser/range.lalrpop";
    println!("cargo:rerun-if-changed={source}");
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .emit_report(true)
        .process_file(source)
        .unwrap();
}

fn build_lalrpop_pql() {
    let source = "src/pql_parser/pql.lalrpop";
    println!("cargo:rerun-if-changed={source}");
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .emit_report(true)
        .process_file(source)
        .unwrap();
}

fn gen_ranking_map(filename: &str, shortdeck: bool, eval: fn(u64) -> i16) {
    let file_path = env::var("OUT_DIR").unwrap() + "/" + filename;

    if Path::new(&file_path).exists() {
        return;
    }

    let mut rankings = FxHashSet::default();

    let iter = CardIter::new(shortdeck);

    for cs in iter.combinations(5) {
        let i = cs.into_iter().fold(0, |a, b| a | b);

        let ranking = eval(i);

        rankings.insert(ranking);
    }

    let mut map_ranking_n = FxHashMap::default();

    for (n, ranking) in rankings.into_iter().sorted().enumerate() {
        let n = u16::try_from(n).unwrap();

        map_ranking_n.insert(ranking, n);
    }

    let data = map_ranking_n
        .write_to_vec()
        .expect("Failed to serialize with speedy");

    fs::write(file_path, data).expect("Unable to write file");
}

fn gen_holdem_abs_ranking_num() {
    gen_ranking_map(
        "holdem-map-ranking-number.bin",
        false,
        prim::eval::holdem5::eval,
    );
}

fn gen_shortdeck_abs_ranking_num() {
    gen_ranking_map(
        "shortdeck-map-ranking-number.bin",
        true,
        prim::eval::shortdeck5::eval,
    );
}

struct CardIter {
    cur: u8,
    shortdeck: bool,
}

impl CardIter {
    pub const fn new(shortdeck: bool) -> Self {
        Self { cur: 0, shortdeck }
    }
}

impl Iterator for CardIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = None;

        while res.is_none() && self.cur < 52 {
            let last_i = self.cur;
            self.cur += 1;

            let rank_idx = last_i % 13;
            let suit_idx = last_i / 13;

            if self.shortdeck && rank_idx <= 4 {
                continue;
            }

            res = Some((1 << rank_idx) << (16 * suit_idx));
        }

        res
    }
}
