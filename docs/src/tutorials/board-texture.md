# Board Texture Studies

This tutorial shows how to measure the distribution of board textures a hand encounters.

## Suit Profile on the River

Starting from a two-tone flop, how many suits appear on the river on average?

```sql
select avg(boardSuitCount(river))
from   game='holdem', hero='As9s', villain='*', board='2s3sJh'
```

With two spades on the flop, the river often stays three-suited and sometimes narrows to two.

## Frequency of Monotone Rivers

How often does the river complete a single-suit board?

```sql
select count(monotoneBoard(river)) as pct_monotone_river
from   game='holdem', hero='*', villain='*', board=''
```

## Pair Frequency by Street

```sql
select count(pairedBoard(flop))  as flop_paired,
       count(pairedBoard(turn))  as turn_paired,
       count(pairedBoard(river)) as river_paired
from   game='holdem', hero='*', villain='*', board=''
```

Each selector produces an independent probability, and together they show how pairing accumulates across streets.

## Straight-Possible Boards

```sql
select count(straightBoard(river)) as pct_straight_possible
from   game='holdem', hero='*', villain='*', board=''
```

## Composing Board Texture with Equity

Hero's equity on flushing rivers, holding suited AK:

```sql
select avg(equity(hero))
from   game='holdem', hero='AwKw', villain='**'
where  flushingBoard(river)
```

## A Texture-Range Filter

Use `boardInRange(...)` to limit the simulation to a specific class of boards:

```sql
select avg(equity(hero))
from   game='holdem', hero='AwKw', villain='**', board='*'
where  boardInRange('AwKxQy****')   -- AKQ-high boards, any rest
```
