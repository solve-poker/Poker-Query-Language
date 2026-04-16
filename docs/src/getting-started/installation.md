# Installation

Open PQL can be used in two ways: as a command-line tool (`opql`) or as a library embedded in a Rust program.

## Requirements

- Rust 1.85 or newer (edition 2024)
- A recent `cargo` toolchain

## Install the CLI

Clone the repository and build the runner crate:

```bash
git clone https://github.com/solve-poker/Poker-Query-Language.git
cd Poker-Query-Language
cargo install --path openpql-runner
```

This installs the `opql` binary into `~/.cargo/bin/`. Check it's on your `PATH`:

```bash
opql --help
```

## Use as a Library

Add the runner crate to your `Cargo.toml`:

```toml
[dependencies]
openpql-runner = "0.1.0"
```

See [Library Usage](../reference/library.md) for integration details.

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
