# Board Texture

Board-texture functions describe the community cards independent of any player's hand. They are typically used inside `count(...)` or as `where` filters.

## Suit profile

### `boardSuitCount(street)`

`TStreet → TCardCount`

Number of distinct suits present on the board at the given street.

```sql
select avg(boardSuitCount(river))
from   game='holdem', hero='As9s', villain='*', board='2s3sJh'
```

### `rainbowBoard(street)`

`TStreet → TBoolean`

`true` if every board card is a different suit (`boardSuitCount(street) = 3` on the flop).

### `twoToneBoard(street)`

`TStreet → TBoolean`

`true` if exactly two suits appear on the board.

### `monotoneBoard(street)`

`TStreet → TBoolean`

`true` if every board card is the same suit.

### `flushingBoard(street)`

`TStreet → TBoolean`

`true` if a flush is *possible* using only the board — i.e. three or more cards share a suit.

## Pairing and straights

### `pairedBoard(street)`

`TStreet → TBoolean`

`true` if at least two board cards share a rank.

### `straightBoard(street)`

`TStreet → TBoolean`

`true` if the board itself contains a 5-card straight.

## Single-card accessors

### `turnCard()`

`→ TCard`

The card dealt on the turn (the fourth board card).

### `riverCard()`

`→ TCard`

The card dealt on the river (the fifth board card).

```sql
select count(turnCard() = toCard('As'))
from   game='holdem', hero='*', villain='*', board='Kh9d2c'
```

## Range membership

### `boardInRange(boardRange)`

`TBoardRange → TBoolean`

`true` if the board (at the river — full 5-card form) lies in the given board-range pattern. Useful for filtering by texture without writing a chain of predicates:

```sql
select avg(equity(hero))
from   game='holdem', hero='AwKw', villain='**'
where  boardInRange('AwKxQyJzTw')   -- straight flush boards in spades, say
```

## Combining

Board functions compose naturally with aggregates and equity. For example:

```sql
-- Hero's equity on flushing rivers, holding suited AK
select avg(equity(hero))
from   game='holdem', hero='AwKw', villain='**'
where  flushingBoard(river)
```
