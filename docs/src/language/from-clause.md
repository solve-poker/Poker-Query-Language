# From Clause

The `from` clause defines the scenario PQL will simulate. Each binding is `key='value'`, with bindings separated by commas. Keys are case-insensitive; values are always single-quoted strings.

```sql
from game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c', dead='2c'
```

A binding key may appear at most once. Duplicate keys produce a parse error.

## Reserved Keys

| Key      | Value type        | Meaning |
| -------- | ----------------- | ------- |
| `game`   | game name         | Which poker variant to play (default `holdem`) |
| `board`  | board range       | Community cards or a board pattern |
| `dead`   | card list         | Cards removed from the deck before dealing |

Anything that is **not** one of those three is interpreted as a **player name** and its value parsed as a [range](./ranges.md).

## `game`

Selects the poker variant. Open PQL currently supports:

- `holdem`     — Texas Hold'em (default)
- `omaha`      — Pot-Limit Omaha (4 hole cards)
- `shortdeck`  — 6+ Hold'em (36-card deck)

See [Supported Games](./games.md) for the full description.

## Players

Players are declared by giving each one a name and a range:

```sql
hero='AhKh'                         -- exact two cards
villain='QQ+'                       -- pocket pair QQ or better
villain1='AwKw, AxKy'               -- a small named range
fish='*'                            -- any two cards
```

Any identifier (other than `game`, `board`, `dead`) is accepted as a player name; the convention is `hero`, `villain`, `villain1`, …, `villainN`. The full set of players in the `from` clause defines the seat lineup for that query.

See [Range Notation](./ranges.md) for the value syntax. **Classic** notation (`AKs`, `AKo`) is not yet implemented; only the **generic** variable-suit syntax is supported.

## `board`

The community-card situation. The board can be a fully-known set of cards or a board range pattern:

```sql
board='Ah9s2c'             -- flop
board='Ah9s2c7d'           -- turn
board='Ah9s2c7dTs'         -- river
board=''                   -- preflop (no community cards)
board='Aw9x2y'             -- generic flop pattern (any rainbow A-9-2)
```

When a partial board is given, remaining streets are sampled per trial. See [Boards and Streets](./boards.md).

## `dead`

Cards that should be removed from the deck before dealing. Useful for "given the burn cards…" scenarios:

```sql
dead='2c, 7h'
```

`dead` cards never appear in any player's holding nor on the board.

## Defaults

| Field    | Default if omitted |
| -------- | ------------------ |
| `game`   | `holdem` |
| `board`  | `*` (preflop, all five board cards sampled) |
| `dead`   | empty |
| players  | (no players declared — usually you want at least one) |

## Ordering

Bindings may appear in any order. By convention, declare `game` first, then players, then `board`, then `dead`.
