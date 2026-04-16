# CLI Basics

The `opql` binary is a thin wrapper around the runner crate. It currently exposes a single entry point, `--run`, that executes a PQL string and writes the result to stdout.

## Usage

```bash
opql --run "<PQL query>"
```

Example:

```bash
opql --run "select equity from hero='AA', villain='KK', board='AhKh2c', game='holdem'"
```

## Notes

- Quote the full query with double quotes so the shell passes it as one argument.
- Use single quotes inside the query for hand, range, and board literals.
- Errors in parsing or evaluation are written to stderr; successful results go to stdout.

## Getting Help

```bash
opql --help
```

For richer workflows — batching queries, scripting with results, custom trial counts — use the [library API](../reference/library.md) directly.
