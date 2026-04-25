# CLI Basics

The `opql` binary is a thin wrapper around the runner crate. It currently exposes a single entry point, `--run`, that executes a PQL string and writes the result to stdout.

## Usage

```bash
opql --run "<PQL query>"
```

Example:

```bash
opql --run "select equity(hero) from game='holdem', hero='AA', villain='KK', board='AhKh2c'"
```

## Notes

- Quote the full query with double quotes so the shell passes it as one argument.
- Use single quotes inside the query for hand, range, board, and game literals.
- Multiple statements may be separated by `;` and will be reported one after another.
- Errors in parsing or evaluation are written to stderr; successful results go to stdout.
- The default trial count is set in `VmStaticData::DEFAULT_N_TRIALS` (60,000 in release builds, 100 in debug builds). It is currently not configurable from the CLI.

## Getting Help

```bash
opql --help
```

For richer workflows — programmatic access to results, custom trial counts, or building queries from data — use the [library API](../reference/library.md) directly.
