# Built-in Functions Overview

PQL ships with a library of poker-specific functions you can use inside `select`. They group loosely into the categories below; each has its own page with signatures and examples.

| Category | Functions | Page |
| --- | --- | --- |
| Equity | `equity`, `hvhequity`, `minHvHEquity`, `riverEquity`, `fractionalRiverEquity` | [Equity](./equity.md) |
| Hand categories | `flopHandCategory`, `exactFlopHandCategory`, `minFlopHandCategory`, `overpair`, `pocketPair` | [Hand Categories](./hand-category.md) |
| Board texture | `boardSuitCount`, `flushingBoard`, `monotoneBoard`, `pairedBoard`, `rainbowBoard`, `twotoneBoard`, `straightBoard`, `turnCard`, `riverCard` | [Board Texture](./board.md) |
| Rank utilities | `boardRanks`, `handRanks`, `maxRank`, `minRank`, `nthRank`, `rankCount`, `hasTopBoardRank`, `hasSecondBoardRank`, `handBoardIntersections`, and more | [Rank Utilities](./rank.md) |
| Outs | `nutHi`, `nutHiForHandType` | [Outs](./outs.md) |

## Common Shapes

Most functions take one or more of:

- **Street**: `flop`, `turn`, `river`
- **Player**: `hero`, `villain`
- **Hand / board expressions**: compose with other functions

Function names are case-insensitive. The examples in this book use camelCase for readability, but `boardSuitCount`, `boardsuitcount`, and `BOARDSUITCOUNT` are all valid.

## Where Functions Live in the Source

The function implementations live under [`openpql-runner/src/functions/`](https://github.com/solve-poker/Poker-Query-Language/tree/main/openpql-runner/src/functions). If the book lags behind, the source is the source of truth.
