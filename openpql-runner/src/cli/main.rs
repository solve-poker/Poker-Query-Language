use std::io;

use clap::Command;
use opql::PQLRunner;

fn build_cli() -> Command {
    Command::new("opql")
        .about("A poker query language CLI\n\n⚠️  WARNING: This project is WIP and subject to change.")
        .arg(clap::Arg::new("command")
            .long("run")
            .value_name("PQL")
            .help("run PQL and exit"))
            .arg_required_else_help(true)
}

fn main() {
    let matches = build_cli().get_matches();

    if let Some(command) = matches.get_one::<String>("command") {
        run_command(command);
    }
}

fn run_command(command: &str) {
    let _ = PQLRunner::run(command, &mut io::stdout(), &mut io::stderr());
}
