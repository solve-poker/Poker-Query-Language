# opql

> ⚠️ **Work in Progress**: This project is currently under active development and is not yet ready for production use.

[![crates.io](https://img.shields.io/crates/v/opql.svg)](https://crates.io/crates/opql)

Command-line interface for Open PQL (Poker Query Language). This binary provides an interactive shell and command-line access to PQL functionality for poker analysis.

## Overview

The `opql` CLI tool allows you to:

- Execute PQL queries from the command line
- Perform poker equity calculations
- Analyze hand ranges and board textures
- Run batch poker analysis tasks
- Interactive PQL shell for exploratory analysis

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd rs-pql

# Build and install
cargo install --path opql
```

### From Crates.io

```bash
cargo install cargo-opql
```

## Usage

### Command-line Queries

Execute single PQL queries directly:

```bash
# Basic equity calculation
opql -c "select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'"

# Average board suit count analysis
opql -c "select avg(boardsuitcount(river)) from hero='As9s', villain='*', board='2s3sJh', game='holdem'"

# Hand type frequency analysis
opql -c "select handtype, count(*) from hero='*', board='AhKs2c', game='holdem' group by handtype"
```

### Interactive Shell (WIP)

Launch the interactive shell for exploratory analysis:

```bash
opql
```

In the interactive shell:

```sql
opql> select equity from hero='AhKh', villain='QQ', board='', game='holdem';
Hero Equity: 56.77%

opql> select avg(nuts) from hero='*', board='AhKs2c7d', game='holdem';
Average Nuts: 0.0234

opql> help
Available commands:
  help     - Show this help message
  quit     - Exit the shell
  clear    - Clear the screen
```

### Command-line Options (WIP)

```bash
opql [OPTIONS]

OPTIONS:
    -c, --command <COMMAND>    Execute a single PQL command and exit
    -f, --file <FILE>         Execute commands from file
    -v, --verbose             Enable verbose output
    -h, --help                Print help information
    -V, --version             Print version information
```

## PQL Query Examples

### Basic Equity Calculations

```sql
-- Preflop equity
select equity from hero='AhKh', villain='QQ', game='holdem';

-- Postflop equity with specific board
select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem';
```

### Range Analysis

```sql
-- Equity vs range
select equity from hero='AhKh', villain='QQ+,AK', board='', game='holdem';

-- Hand strength distribution
select handtype, count(*) from hero='*', board='AhKs2c' group by handtype;
```

### Board Analysis

```sql
-- Board texture analysis
select avg(boardsuitcount(flop)), avg(paired(flop)) from board='*';

-- Turn/river analysis
select avg(equity) from hero='AA', villain='*', board='Ah9s2c*', game='holdem';
```

### Advanced Queries

```sql
-- Multi-street analysis
select street, avg(equity) from hero='AhKh', villain='22+', board='Ah9s2c*' group by street;

-- Conditional analysis
select equity from hero='AhKh', villain='*' where inrange(villain, 'QQ+,AK');
```

## Configuration

The CLI tool supports configuration through:

- Environment variables
- Configuration files
- Command-line arguments

See the main project documentation for detailed configuration options.

## Development

### Building

```bash
cargo build --package opql
```

### Testing

```bash
cargo test --package opql
```

## License

Licensed under the MIT License. See [LICENSE](../LICENSE) for details.
