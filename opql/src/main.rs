use std::io;

use clap::Command;
use open_pql::StatementsRunner;

fn build_cli() -> Command {
    Command::new("opql")
        .about("A poker query language CLI\n\n⚠️  WARNING: This project is WIP and subject to change.")
        .arg(clap::Arg::new("command")
            .short('c')
            .long("command")
            .value_name("COMMAND")
            .help("run only single PQL command and exit"))
            .arg_required_else_help(true)
}

#[cfg(not(debug_assertions))]
const N: usize = 600_000;

#[cfg(not(debug_assertions))]
const T: usize = 8;

#[cfg(debug_assertions)]
const N: usize = 6000;

#[cfg(debug_assertions)]
const T: usize = 1;

fn main() {
    let matches = build_cli().get_matches();

    if let Some(command) = matches.get_one::<String>("command") {
        run_command(command);
    }
}

fn run_command(command: &str) {
    let mut r = StatementsRunner::new(
        command,
        N,
        T,
        Box::new(Vec::<u8>::new()),
        Box::new(Vec::<u8>::new()),
    );

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
