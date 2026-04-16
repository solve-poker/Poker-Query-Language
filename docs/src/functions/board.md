# Board Texture

Board texture functions describe what the community cards look like, independent of any player's hand.

## `boardSuitCount(street)`

The number of distinct suits present on the board at the given street.

```sql
select avg(boardSuitCount(river))
from   hero='As9s', villain='*', board='2s3sJh', game='holdem'
```

## `rainbowBoard(street)` / `twotoneBoard(street)` / `monotoneBoard(street)`

Boolean predicates for three-suit, two-suit, and one-suit boards.

## `flushingBoard(street)`

True if the board itself contains three or more cards of the same suit (i.e. a flush is possible on the board alone).

## `pairedBoard(street)`

True if at least two board cards share a rank.

## `straightBoard(street)`

True if the board itself contains a made straight.

## `turnCard` / `riverCard`

The single card dealt on the turn or river. Useful as the argument to rank or suit operators.

```sql
select count(suit(riverCard) = 's')
from   hero='AsKs', villain='*', board='2s3h7d', game='holdem'
```

## Combining

Board functions compose naturally with aggregates and equity. For example, "what is hero's equity on flushing rivers?" can be written as a `riverEquity` aggregated under a filter on `flushingBoard(river)`.
