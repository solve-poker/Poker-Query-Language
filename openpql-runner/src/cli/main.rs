use std::io;

use clap::Command;
use opql::PQLRunner;

fn build_cli() -> Command {
    Command::new("opql")
        .about(
            "A poker query language CLI\n\n⚠️  WARNING: This project is WIP and subject to change.",
        )
        .arg(
            clap::Arg::new("command")
                .long("run")
                .value_name("PQL")
                .help("run PQL and exit"),
        )
        .arg(
            clap::Arg::new("max_trials")
                .long("mt")
                .value_name("N")
                .value_parser(clap::value_parser!(usize))
                .help("max number of trials per statement"),
        )
        .arg(
            clap::Arg::new("n_threads")
                .long("threads")
                .value_name("N")
                .value_parser(clap::value_parser!(usize))
                .help("number of worker threads (defaults to available cores)"),
        )
        .arg_required_else_help(true)
}

fn main() {
    let matches = build_cli().get_matches();
    let max_trials = matches.get_one::<usize>("max_trials").copied();
    let n_threads = matches.get_one::<usize>("n_threads").copied();

    if let Some(command) = matches.get_one::<String>("command") {
        run_command(command, max_trials, n_threads);
    }
}

fn run_command(command: &str, max_trials: Option<usize>, n_threads: Option<usize>) {
    let _ = PQLRunner::run(
        command,
        max_trials,
        n_threads,
        &mut io::stdout(),
        &mut io::stderr(),
    );
}
