# Rank Utilities

Rank utilities reason about which ranks (2 through A) appear in a hand or on the board. They return either a `TRankSet`, a single `TRank`, or a count.

## Set-returning

### `boardRanks(street)`

`TStreet → TRankSet`

The set of distinct ranks present on the board at the given street.

### `handRanks(player, street)`

`TPlayer × TStreet → TRankSet`

The set of distinct ranks in the player's hole cards. (For Hold'em the answer doesn't depend on the street; for stud variants — when added — it would.)

### `duplicatedBoardRanks(street)`

`TStreet → TRankSet`

The set of ranks that appear more than once on the board (e.g. on a paired board).

### `duplicatedHandRanks(player, street)`

`TPlayer × TStreet → TRankSet`

Ranks that appear more than once in the player's hand. For Omaha this lets you spot pocket pairs and trips inside the four hole cards.

### `intersectingHandRanks(player, street)`

`TPlayer × TStreet → TRankSet`

Hand ranks that also appear on the board.

### `nonIntersectingHandRanks(player, street)`

`TPlayer × TStreet → TRankSet`

Hand ranks that do **not** appear on the board.

## Scalar

### `maxRank(ranks)`

`TRankSet → TRank`

Highest rank in the set. Errors on an empty set.

### `minRank(ranks)`

`TRankSet → TRank`

Lowest rank in the set. Errors on an empty set.

### `nthRank(n, ranks)`

`TInteger × TRankSet → TRank`

The `n`-th highest rank in the set, 1-indexed. For instance, on `boardRanks(flop) = {A, K, J}`:

```text
nthRank(1, boardRanks(flop)) = A
nthRank(2, boardRanks(flop)) = K
nthRank(3, boardRanks(flop)) = J
```

### `rankCount(ranks)`

`TRankSet → TCardCount`

Cardinality of the set (number of distinct ranks).

### `handBoardIntersections(player, street)`

`TPlayer × TStreet → TCardCount`

How many of the player's hole cards share a rank with at least one board card.

## Predicates

### `hasTopBoardRank(player, street)`

`TPlayer × TStreet → TBoolean`

`true` if the player has at least one card matching the highest rank on the board at the given street.

### `hasSecondBoardRank(player, street)`

`TPlayer × TStreet → TBoolean`

`true` if the player has at least one card matching the second-highest board rank. Returns `false` on a board with only one distinct rank.

## Example

Frequency hero flops top pair:

```sql
select count(hasTopBoardRank(hero, flop)) as pct_toppair
from   game='holdem', hero='AxKy', villain='*', board=''
```

Average highest rank on the river:

```sql
select avg(maxRank(boardRanks(river)))
from   game='holdem', hero='*', villain='*'
```
