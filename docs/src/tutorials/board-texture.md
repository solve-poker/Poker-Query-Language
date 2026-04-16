# Board Texture Studies

This tutorial shows how to measure the distribution of board textures a hand encounters.

## Suit Profile on the River

Starting from a two-tone flop, how many suits appear on the river on average?

```sql
select avg(boardSuitCount(river))
from   hero='As9s', villain='*', board='2s3sJh', game='holdem'
```

With two spades on the flop, the river board often stays three-suited, sometimes narrows to two.

## Frequency of Monotone Rivers

How often does the river complete a monotone (single-suit) board?

```sql
select count(monotoneBoard(river)) as pct_monotone_river
from   hero='*', villain='*', board='', game='holdem'
```

## Paired vs Unpaired

Pair-frequency on the turn:

```sql
select count(pairedBoard(turn)) as pct_paired_turn
from   hero='*', villain='*', board='', game='holdem'
```

## Comparing Streets

You can ask multiple questions in one query:

```sql
select count(pairedBoard(flop)) as flop_paired,
       count(pairedBoard(turn)) as turn_paired,
       count(pairedBoard(river)) as river_paired
from   hero='*', villain='*', board='', game='holdem'
```

Each selector produces an independent probability, and together they show how pairing accumulates across streets.
