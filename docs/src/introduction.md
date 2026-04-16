# Introduction

**Open PQL** is a high-performance Rust implementation of the Poker Query Language (PQL), enabling SQL-like queries for poker analysis and calculations. It is a spiritual successor to the original Java implementation developed by Odds Oracle.

> ⚠️ **Work in Progress**: This project is under active development and is not yet ready for production use.

## What is PQL?

PQL lets you ask questions about poker situations in a declarative, SQL-like syntax:

```sql
select equity
from   hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'
```

This computes hero's equity against villain's range on a given flop.

## Why Open PQL?

- **Familiar syntax** — SQL-like grammar is easy to learn if you know databases.
- **High performance** — written in Rust, with Monte Carlo sampling for fast estimates.
- **Multi-game support** — Texas Hold'em and other variants.
- **Library + CLI** — use the `opql` command or embed the runner crate in your code.

## Workspace Crates

| Crate | Purpose |
| --- | --- |
| `openpql-prelude` | Core poker types: cards, hands, evaluators |
| `openpql-core` | Game abstraction and query execution core |
| `openpql-range-parser` | Parser for range notation (`AA-TT`, `AwKw+`, ...) |
| `openpql-pql-parser` | Parser for PQL syntax |
| `openpql-runner` | Query executor and `opql` CLI |
| `openpql-macro` | Internal procedural macros |

## How to Read This Book

- If you're new, start with [Installation](./getting-started/installation.md) then [Your First Query](./getting-started/first-query.md).
- The [PQL Language](./language/query-structure.md) section covers syntax in depth.
- [Built-in Functions](./functions/overview.md) lists every function available inside `select`.
- The [Tutorials](./tutorials/preflop-equity.md) walk through realistic analysis workflows.

## Try It Online

An interactive demo is available at <https://pql-playground.solve.poker>.
