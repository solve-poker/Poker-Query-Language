# From Clause

The `from` clause defines the scenario PQL will simulate. Each binding is `name=value` where the value is quoted.

## Standard Bindings

### `game`

Selects the poker variant:

```sql
game='holdem'
```

See [Supported Games](./games.md) for the full list.

### `hero` and `villain`

Declare each player's holding or range:

```sql
hero='AhKh'        -- a specific two-card hand
villain='QQ+'      -- a range: any pocket pair QQ or better
villain='*'        -- any two cards (fully random)
```

Multiple villains can be declared by repeating the name, or by using `villain1`, `villain2`, etc. depending on the scenario being modeled. See [Range Notation](./ranges.md) for the full syntax of the right-hand-side string.

### `board`

The community cards dealt so far. The length of the string determines the street the simulation starts from:

- 0 cards (`board=''` or omitted): preflop
- 3 cards: flop
- 4 cards: turn
- 5 cards: river (no further dealing)

```sql
board='Ah9s2c'          -- flop
board='Ah9s2c7d'        -- turn
board='Ah9s2c7dTs'      -- river
```

## Ordering

Bindings may appear in any order. By convention, declare `hero` first, then villains, then `board`, then `game`.
