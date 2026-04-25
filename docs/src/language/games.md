# Supported Games

The `game='…'` binding selects the poker variant. Each variant changes the deck, the number of hole cards, and the hand evaluator.

| Value       | Variant            | Hole cards | Deck         |
| ----------- | ------------------ | ---------- | ------------ |
| `holdem`    | Texas Hold'em      | 2          | Full 52      |
| `omaha`     | Pot-Limit Omaha    | 4          | Full 52      |
| `shortdeck` | Short-Deck Hold'em | 2          | 36 (6s–As)   |

`holdem` is the default if `game` is omitted. Open PQL is currently a **Hi-only** implementation — Hi/Lo splits (Omaha 8, Stud 8) and stud variants (Stud Hi, Razz) are not supported.

## Hold'em

Players are dealt two hole cards, share a five-card board, and use any combination of seven cards to make the best five-card hand.

```sql
select equity(hero)
from   game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'
```

## Omaha

Four hole cards per player. Each player **must** use exactly two of their hole cards and three of the board cards. Range strings still use the same notation; concrete hands require four cards (e.g. `AhAsKhKs`):

```sql
select equity(hero)
from   game='omaha', hero='AhAsKhKs', villain='**'
```

## Short Deck

A 36-card deck (deuces through fives removed). Common Short-Deck rule choices apply: A-6-7-8-9 is the wheel straight, and flushes beat full houses. The prelude crate's evaluator implements the standard ranking.

```sql
select equity(hero)
from   game='shortdeck', hero='AwAx', villain='**'
```

## One Game per Query

Each query targets a single game. You cannot mix variants inside one query.
