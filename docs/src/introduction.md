# Introduction

**Open PQL** is a high-performance Rust implementation of the **Poker Query Language** (PQL), enabling SQL-like queries for poker probability analysis. It is a spiritual successor to the original Java implementation that powers Odds Oracle and propokertools.com.

> ⚠️ **Work in Progress**: this project is under active development. Public APIs, syntax, and supported features may change.

## What is PQL?

PQL lets you ask poker questions in a declarative, SQL-like syntax:

```sql
select avg(equity(hero))
from   game='holdem', hero='AwAx', villain='*'
```

Read it as: "compute hero's average all-in equity holding any pair of aces against a random hand in Hold'em."

PQL is intended for people with a decent level of poker and technical sophistication who want to explore probability questions without writing custom programs.

## What kinds of questions can PQL answer?

A few examples to give you the flavour:

Hero's equity on a known flop against a tight range:

```sql
select equity(hero)
from   game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'
```

How often hero flops a set with a small pair, given villain has any two cards:

```sql
select count(flopHandCategory(hero) = flopset) as pct_set
from   game='holdem', hero='TT', villain='*'
```

How often the river completes a monotone (single-suit) board:

```sql
select count(monotoneBoard(river)) as pct_monotone
from   game='holdem', hero='*', villain='*'
```

## Why "PQL"?

The syntax is loosely based on **SQL**: it borrows `SELECT`, `FROM`, `WHERE`, `AS`, `AND`, `OR`, `NOT`, the comparison operators (`=`, `<`, `<=`, `>`, `>=`), arithmetic (`+`, `-`, `*`, `/`), and parentheses for grouping. If you've written SQL before, the shape will look familiar.

## Workspace Crates

| Crate | Purpose |
| --- | --- |
| `openpql-prelude` | Core poker types: cards, hands, evaluators, game variants |
| `openpql-core` | Game abstraction and shared compute kernels |
| `openpql-range-parser` | Parser for the (generic) range notation, e.g. `AwKw, 77-55` |
| `openpql-pql-parser` | Parser for PQL syntax (LALRPOP grammar) |
| `openpql-runner` | Query executor and the `opql` CLI |
| `openpql-macro` | Internal procedural macros for function registration |

## How to Read This Book

- Start with [Installation](./getting-started/installation.md), then [Your First Query](./getting-started/first-query.md).
- The [PQL Language](./language/query-structure.md) section covers selectors, the `from`/`where` clauses, ranges, boards, games, and types.
- [Built-in Functions](./built-ins/overview.md) lists every function available inside `select` with its argument and return types.
- The [Tutorials](./tutorials/preflop-equity.md) walk through realistic analysis workflows.
- The [Reference](./reference/library.md) section covers embedding the runner from a Rust program.

## Try It Online

An interactive demo is available at <https://pql-playground.solve.poker>.
