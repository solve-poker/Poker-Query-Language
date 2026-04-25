# Hand Categories

These functions classify a player's made hand: the broad five-card category, the more detailed flop category, and ratings for comparing hand strengths.

See [Types](../language/types.md) for the full list of `THandType` and `TFlopHandCategory` values.

## Flop hand categories

### `flopHandCategory(player)`

`TPlayer → TFlopHandCategory`

The general flop hand category for `player`. Gives more specific information than `handType` — for instance, distinguishing top pair from middle pair, or flopping a set vs. trips.

```sql
select count(flopHandCategory(hero) = floptoppair) as pct_toppair
from   game='holdem', hero='AwKw', villain='*', board='As7d2c'
```

### `exactFlopHandCategory(player, category)`

`TPlayer × TFlopHandCategory → TBoolean`

Returns `true` if the player has exactly the given category on the flop. Equivalent to `flopHandCategory(player) = category`, expressed as a single function call.

### `minFlopHandCategory(player, category)`

`TPlayer × TFlopHandCategory → TBoolean`

Returns `true` if the player's flop category is at least as strong as the given one (categories are ordered).

## Five-card hand types

### `handType(player, street)`

`TPlayer × TStreet → THandType`

Returns the player's best 5-card hand category on the given street. Output is one of `highcard`, `pair`, `twopair`, `trips`, `straight`, `flush`, `fullhouse`, `quads`, `straightflush`.

```sql
select count(handType(hero, river) = flush) as pct_flush_river
from   game='holdem', hero='AwKw', villain='*', board=''
```

### `exactHandType(player, street, type)`

`TPlayer × TStreet × THandType → TBoolean`

Returns `true` if the player has *exactly* the given hand type on that street.

### `minHandType(player, street, type)`

`TPlayer × TStreet × THandType → TBoolean`

Returns `true` if the player has at least the given hand type. Useful for "top pair or better" filters:

```sql
select count(minHandType(hero, flop, pair)) as pct_pair_or_better
from   game='holdem', hero='AxKy', villain='*', board=''
```

### `winningHandType()`

`→ THandType`

The hand type of the winning hand on the river. If the pot is split between hands of equal type, the shared type is returned.

```sql
select count(winningHandType() = flush) as pct_winner_was_flush
from   game='holdem', hero='*', villain='*'
```

## Pocket-pair predicates

### `pocketPair(player)`

`TPlayer → TBoolean`

`true` if the player's hole cards contain at least two cards of the same rank.

### `overpair(player, street)`

`TPlayer × TStreet → TBoolean`

`true` if the player has a pocket pair strictly higher than every rank on the board at the given street.

```sql
select count(overpair(hero, flop)) as pct_overpair
from   game='holdem', hero='QQ+', villain='*', board='Jc7d2s'
```

## Ratings

Ratings are opaque numeric scores you use **only to compare hands within the same game**. The absolute values have no external meaning.

### `hiRating(player, street)`

`TPlayer × TStreet → THiRating`

Returns a rating representing the strength of the player's best hi hand on the given street. Useful when comparing hand strengths over a sequence of streets.

### `bestHiRating(player, street)`

`TPlayer × TStreet → TBoolean`

`true` if no other declared player has a strictly better hi hand on the given street.

### `maxHiRating(player, street)`

`TPlayer × TStreet → TBoolean`

`true` if the player has the maximum hi rating across declared players (ties allowed).

### `minHiRating(player, street, rating)`

`TPlayer × TStreet × THiRating → TBoolean`

`true` if the player's hi rating equals or exceeds the given rating value.

### `rateHiHand(cards)`

`TString → THiRating`

Given a 5-card string (e.g. `'AsKsQsJsTs'`), returns its hi rating. Useful for building thresholds to feed into `minHiRating`.

```sql
select count(minHiRating(hero, river, rateHiHand('AsAdKsKdQc'))) as pct_aa_kk_or_better
from   game='holdem', hero='*', villain='*'
```
