# Supported Games

The `game='…'` binding selects the poker variant. Each variant changes the number of hole cards and the hand evaluator.

| Value | Variant | Hole cards | Deck |
| --- | --- | --- | --- |
| `holdem` | Texas Hold'em | 2 | Full 52 |
| `omaha` | Omaha Hi | 4 | Full 52 |
| `shortdeck` | Short-Deck Hold'em | 2 | 36 (6s–As) |

## Holdem

The default. Players are dealt two hole cards, share a five-card board, and use any combination of seven cards to make the best five-card hand.

## Omaha

Four hole cards per player, but each player **must** use exactly two from their hand and three from the board. Range strings still use the same notation; hand literals require four cards (e.g. `AhAsKhKs`).

## Short Deck

A 36-card deck (deuces through fives removed). Straights use A-6-7-8-9 as the low straight, and flushes beat full houses in some rulesets. The prelude crate's evaluator implements the common short-deck ranking.

## Changing the Game

Each query picks a single game. You cannot mix variants inside one query.

```sql
select equity
from   hero='AhAsKhKs', villain='*', board='', game='omaha'
```
