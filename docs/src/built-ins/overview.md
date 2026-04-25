# Built-in Functions Overview

Open PQL ships with a library of poker-specific functions you can use inside `select` and `where`. They group loosely into the categories below; each has its own page with descriptions and examples.

| Category | Page |
| -------- | ---- |
| Equity | [Equity](./equity.md) |
| Hand categories, types, and ratings | [Hand Categories](./hand-category.md) |
| Board texture | [Board Texture](./board.md) |
| Rank utilities | [Rank Utilities](./rank.md) |
| Outs | [Outs](./outs.md) |
| Outcomes & helpers | [Outcomes](./outcomes.md) |

## Common shapes

Most functions take some combination of:

- **`TPlayer`** — an identifier declared in the `from` clause (`hero`, `villain`, …)
- **`TStreet`** — one of `preflop`, `flop`, `turn`, `river`
- **`THandType`** or **`TFlopHandCategory`** — a hand-class keyword
- **`TRange`** / **`TBoardRange`** — a single-quoted range string
- **`TRankSet`** — typically the result of `boardRanks(...)` or `handRanks(...)`

Function names are **case-insensitive**. The book uses camelCase for readability, but `boardSuitCount`, `boardsuitcount`, and `BOARDSUITCOUNT` all parse the same.

## Full function index

The 49 unique functions currently implemented (with two extra aliases) are:

| Function | Argument types | Return type |
| -------- | -------------- | ----------- |
| `bestHiRating` | TPlayer, TStreet | TBoolean |
| `boardInRange` | TBoardRange | TBoolean |
| `boardRanks` | TStreet | TRankSet |
| `boardSuitCount` | TStreet | TCardCount |
| `duplicatedBoardRanks` | TStreet | TRankSet |
| `duplicatedHandRanks` | TPlayer, TStreet | TRankSet |
| `equity` (alias of `hvhEquity`) | TPlayer, TStreet | TEquity |
| `exactFlopHandCategory` | TPlayer, TFlopHandCategory | TBoolean |
| `exactHandType` | TPlayer, TStreet, THandType | TBoolean |
| `flopHandCategory` | TPlayer | TFlopHandCategory |
| `flushingBoard` | TStreet | TBoolean |
| `fractionalRiverEquity` | TPlayer | TFraction |
| `handBoardIntersections` | TPlayer, TStreet | TCardCount |
| `handRanks` | TPlayer, TStreet | TRankSet |
| `handType` | TPlayer, TStreet | THandType |
| `hasSecondBoardRank` | TPlayer, TStreet | TBoolean |
| `hasTopBoardRank` | TPlayer, TStreet | TBoolean |
| `hiRating` | TPlayer, TStreet | THiRating |
| `hvhEquity` | TPlayer, TStreet | TEquity |
| `inRange` | TPlayer, TRange | TBoolean |
| `intersectingHandRanks` | TPlayer, TStreet | TRankSet |
| `maxHiRating` | TPlayer, TStreet | TBoolean |
| `maxRank` | TRankSet | TRank |
| `minEquity` (alias of `minHvHEquity`) | TPlayer, TStreet, TDouble | TBoolean |
| `minFlopHandCategory` | TPlayer, TFlopHandCategory | TBoolean |
| `minHandType` | TPlayer, TStreet, THandType | TBoolean |
| `minHiRating` | TPlayer, TStreet, THiRating | TBoolean |
| `minHvHEquity` | TPlayer, TStreet, TDouble | TBoolean |
| `minRank` | TRankSet | TRank |
| `monotoneBoard` | TStreet | TBoolean |
| `nonIntersectingHandRanks` | TPlayer, TStreet | TRankSet |
| `nthRank` | TInteger, TRankSet | TRank |
| `nutHi` | TPlayer, TStreet | TBoolean |
| `nutHiForHandType` | TPlayer, TStreet | TBoolean |
| `overpair` | TPlayer, TStreet | TBoolean |
| `pairedBoard` | TStreet | TBoolean |
| `pocketPair` | TPlayer | TBoolean |
| `rainbowBoard` | TStreet | TBoolean |
| `rankCount` | TRankSet | TCardCount |
| `rateHiHand` | TString | THiRating |
| `riverCard` |  | TCard |
| `riverEquity` | TPlayer | TEquity |
| `scoops` | TPlayer | TBoolean |
| `straightBoard` | TStreet | TBoolean |
| `tiesHi` | TPlayer | TBoolean |
| `toCard` | TString | TCard |
| `toRank` | TString | TRank |
| `turnCard` |  | TCard |
| `twoToneBoard` | TStreet | TBoolean |
| `winningHandType` |  | THandType |
| `winsHi` | TPlayer | TBoolean |

## Functions in the original PQL spec but not yet implemented

For users coming from the original Java PQL, the following functions are documented in the upstream spec but **not yet** available in Open PQL:

- All Lo-hand functions: `bestLoRating`, `boardAllowsMadeLo`, `boardHasOneDistinctLoCard`, `boardHasTwoDistinctLoCards`, `boardLoCardCount`, `loRating`, `madeLo`, `minLoRating`, `nutLo`, `nutLoOuts`, `rateLoHand`, `tiesLo`, `winsLo`
- Multi-opponent random-range equities: `HvREquity`, `HvPerceivedRangeEquity`, `minHvREquity`, `minHvPerceivedRangeEquity`
- Hand-strength helpers: `fiveCardHiHandNumber`, `handRanking`, `handRankingFor`, `cardsPlay`, `upCard`, `outsToHandType`, `minOutsToHandType`, `nutHiOuts`, `fourFlush`, `threeFlush`, `toString`
- The higher-order `handsHaving` selector

## Where the implementations live

The function implementations live under `openpql-runner/src/functions/`. If the book lags behind, the source is the source of truth.
