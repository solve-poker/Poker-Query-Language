# Selectors

A **selector** is an aggregate that reduces an inner expression evaluated over each trial into a single report value. Every PQL `select` list is one or more selectors separated by commas.

PQL supports four selectors:

| Selector | Description | Example |
| --- | --- | --- |
| `avg(expr)`   | Mean of a numeric expression across all trials | `avg(equity(hero))` |
| `count(pred)` | Fraction of trials for which a boolean expression is true (a probability) | `count(wins(hero))` |
| `max(expr)`   | Largest value of an expression seen across trials | `max(handType(hero, river))` |
| `min(expr)`   | Smallest value of an expression seen across trials | `min(fractionalRiverEquity(villain))` |

> Note: a `histogram` selector exists in the original PQL spec but is **not yet implemented** in Open PQL.

## Combining Selectors

A query can ask for any number of selectors in a single shot:

```sql
select count(wins(hero))                 as heroWon,
       avg(equity(hero))                 as heroEv,
       count(handType(hero, river) = flush) as pctFlush
from   game='holdem', hero='AwKw', villain='**'
```

Each selector is reported on its own line.

## Aliases (`as`)

Use `as` to give a selector a readable name:

```sql
select avg(boardSuitCount(river)) as river_suits
from   game='holdem', hero='As9s', villain='*', board='2s3sJh'
```

If `as` is omitted, a default name is generated from the expression.

## Inner Expressions

The expression inside a selector can be any combination of:

- **Function calls** — `equity(hero)`, `handType(hero, flop)`, `boardSuitCount(river)`, …
- **Constants** — numbers, single-quoted strings, hand-type and category keywords (`pair`, `flopset`, …)
- **Comparisons and boolean operators** — `handType(hero, river) = flush`, `equity(hero) > 0.5 and hasTopBoardRank(hero, flop)`
- **Arithmetic** — `equity(hero) - equity(villain)`

See [Built-in Functions](../built-ins/overview.md) for the available primitives.

## Common Recipes

Probability of an event:

```sql
select count(<predicate>) from ...
```

Average value of a metric:

```sql
select avg(<numeric expression>) from ...
```

Worst-case value over the simulation:

```sql
select min(<numeric expression>) from ...
```
