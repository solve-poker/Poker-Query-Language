# Types

PQL is dynamically typed at the surface, but every expression has a known PQL type that determines what other expressions it can combine with. You usually don't need to think about types explicitly — function signatures will guide you — but the table below is useful when chasing a type-mismatch error.

## Scalar Types

| Type                | Meaning |
| ------------------- | ------- |
| `TBoolean`          | `true` or `false` |
| `TInteger` / `TLong`| A whole number |
| `TDouble`           | A double-precision floating point number |
| `TFraction`         | An exact fraction such as `1/2`, `2/5`, `13/914` |
| `TEquity`           | A `TDouble` between 0.0 and 1.0 |
| `TNumeric`          | Any of the numeric types above |
| `TString`           | A single-quoted string literal |

## Card / Hand Types

| Type                | Meaning |
| ------------------- | ------- |
| `TCard`             | A single card (e.g. the Jack of Diamonds) |
| `TCardCount`        | An integer between 0 and 52 |
| `TRank`             | A rank (an Ace, a Ten, a Deuce, …) |
| `TRankSet`          | A set of unique ranks |
| `TStreet`           | One of `preflop`, `flop`, `turn`, `river` |
| `THandType`         | A 5-card hand category (see below) |
| `TFlopHandCategory` | A flop-specific hand category (see below) |
| `THiRating`         | A hi-hand rating, used for comparing hand strength |

## Players and Ranges

| Type           | Meaning |
| -------------- | ------- |
| `TPlayer`      | A player declared in the `from` clause (e.g. `hero`, `villain`) |
| `TPlayerCount` | An integer between 0 and the number of players |
| `TRange`       | A range expression such as `'AwKw, 77-55'` |
| `TBoardRange`  | A range expression for the board, e.g. `'AwKx2y'` |

## `THandType`

The 5-card hand category for a player on a given street.

- `highcard`
- `pair`
- `twopair`
- `trips`
- `straight`
- `flush`
- `fullhouse`
- `quads`
- `straightflush`

Hand-type values are bare keywords inside expressions: `handType(hero, river) = flush`.

## `TFlopHandCategory`

A finer-grained classification of what a flop hand looks like. Useful for "did I flop bottom two?" style filters.

| Value               | Meaning |
| ------------------- | ------- |
| `flopnothing`       | No pair, no draws of consequence (made nothing) |
| `flopunderpair`     | Pocket pair lower than every flop card |
| `flopthirdpair`     | Hits the third (lowest) flop rank |
| `floppocket23`      | Pocket pair between the second and third flop ranks |
| `flopsecondpair`    | Hits the second flop rank |
| `floppocket12`      | Pocket pair between the first and second flop ranks |
| `floptoppair`       | Hits the top flop rank |
| `flopoverpair`      | Pocket pair larger than every flop card |
| `flopbottomtwo`     | Hits the two lowest of three distinct flop ranks |
| `floptopandbottom`  | Hits the top and bottom of three distinct flop ranks |
| `floptoptwo`        | Hits the top two of three distinct flop ranks |
| `floptrips`         | Three of a kind on a paired flop |
| `flopset`           | Three of a kind on an unpaired flop |
| `flopstraight`      | Made a straight |
| `flopflush`         | Made a flush |
| `flopfullhouse`     | Made a full house (takes precedence over pocket-pair categories) |
| `flopquads`         | Four of a kind |
| `flopstraightflush` | Straight flush |

## Notes on Lo Types

The original PQL spec defines `TLoRating` and a number of Lo-hand functions. Open PQL has the type symbol reserved but **no Lo functions are implemented yet**, so `TLoRating` should be treated as a placeholder.
