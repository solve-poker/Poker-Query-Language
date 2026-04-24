<h1 align="center">Open PQL</h1>

<p align="center">
  <strong>SQL for poker. Written in Rust.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/openpql-runner"><img src="https://img.shields.io/crates/v/openpql-runner.svg?logo=rust" alt="crates.io"></a>
  <a href="https://docs.rs/openpql-runner"><img src="https://img.shields.io/docsrs/openpql-runner?logo=docs.rs" alt="docs.rs"></a>
  <a href="https://crates.io/crates/openpql-runner"><img src="https://img.shields.io/crates/d/openpql-runner.svg" alt="downloads"></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.85+-blue.svg?logo=rust" alt="Rust 1.85+"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
</p>

<p align="center">
  <a href="https://pql-playground.solve.poker/"><img src="https://img.shields.io/badge/playground-live-brightgreen.svg" alt="Playground"></a>
  <a href="https://pql-docs.solve.poker"><img src="https://img.shields.io/badge/docs-online-blue.svg" alt="PQL Docs"></a>
</p>

<p align="center">
  <a href="https://pql-playground.solve.poker/">Playground</a> •
  <a href="https://pql-docs.solve.poker">Docs</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#workspace">Workspace</a>
</p>

---

> ⚠️ **Work in progress.** Active development; not yet production-ready.

A high-performance Rust implementation of **Poker Query Language**, the SQL-like language for poker analysis. Spiritual successor to the original Java engine by Odds Oracle.

```sql
select equity
from   hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'
```

## Features

- ♠️ **Familiar syntax** — SQL-like queries for equity, ranges, and board analysis
- ⚡ **Fast** — bit-packed cards, Monte Carlo sampling, optimized evaluators
- 🃏 **Multi-game** — Texas Hold'em, Omaha, Short Deck
- 🎯 **Expressive ranges** — variable-based notation (`AwKw`, `QQ+`, `AwJw+`)
- 🛠️ **Library + CLI** — embed the runner or use the `opql` binary

## Quick Start

Add to `Cargo.toml`:

```toml
[dependencies]
openpql-runner = "0.1"
```

Or use the CLI:

```bash
opql --run "select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'"
```

Try it in your browser at **[pql-playground.solve.poker](https://pql-playground.solve.poker/)**.

## Workspace

| Crate | Purpose |
| --- | --- |
| [`openpql-runner`](./openpql-runner) | Library and `opql` CLI — start here |
| [`openpql-pql-parser`](./openpql-pql-parser) | PQL syntax parser |
| [`openpql-range-parser`](./openpql-range-parser) | Range notation parser |
| [`openpql-core`](./openpql-core) | Game abstraction & execution core |
| [`openpql-prelude`](./openpql-prelude) | Cards, hands, evaluators |
| [`openpql-macro`](./openpql-macro) | Internal proc macros |

## Documentation

- 📖 **[PQL Guide & Tutorial](https://pql-docs.solve.poker)** — language reference and walkthroughs
- 🎮 **[Playground](https://pql-playground.solve.poker/)** — run queries in your browser

## License

MIT — see [LICENSE](LICENSE).

## Acknowledgments

Thanks to the Odds Oracle ([propokertools.com](https://propokertools.com)) team for pioneering PQL.

<p align="center">Made with ♥️</p>
