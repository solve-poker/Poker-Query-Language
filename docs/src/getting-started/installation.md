# Installation

Open PQL ships as a Cargo workspace. You can use it as a command-line tool (`opql`) or embed the runner crate in your own Rust program.

## Requirements

- Rust 1.85 or newer (edition 2024)
- A recent `cargo` toolchain

## Install the CLI

Clone the repository and install the runner crate's binary:

```bash
git clone https://github.com/solve-poker/Poker-Query-Language.git
cd Poker-Query-Language
cargo install --path openpql-runner --features cli
```

This installs the `opql` binary into `~/.cargo/bin/`. Verify that it's on your `PATH`:

```bash
opql --help
```

## Use as a Library

Add the runner crate to your `Cargo.toml`:

```toml
[dependencies]
openpql-runner = "0.1"
```

The library entry point is `opql::PQLRunner`. See [Library Usage](../reference/library.md) for integration details.

## Build From Source

If you plan to contribute or run tests, clone the workspace and use the provided `justfile`:

```bash
just build      # cargo build
just test       # cargo nextest run
just lint       # cargo clippy
just doc        # cargo doc --no-deps
```

## Next Step

Continue to [Your First Query](./first-query.md).
