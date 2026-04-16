# Selectors

A selector is an expression that produces a value for each simulation trial. Multiple selectors are separated by commas and may be aliased with `as`.

## Scalar Selectors

A scalar selector returns one value per trial. The simplest is `equity`, which yields hero's equity share on that trial.

```sql
select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'
```

## Aggregates

Aggregates reduce many trials into a single number.

| Aggregate | Result |
| --- | --- |
| `avg(expr)` | Mean of `expr` across trials |
| `count(predicate)` | Probability of `predicate` being true |

Example — frequency the river is a spade:

```sql
select count(riversuit = 's') as pct_spade_river
from   hero='AsKs', villain='*', board='2s3h7d', game='holdem'
```

## Aliases

Use `as` to give a selector a readable name:

```sql
select avg(boardsuitcount(river)) as river_suits
from   hero='As9s', villain='*', board='2s3sJh', game='holdem'
```

Aliases must be unique inside a query.

## Nesting

Functions can be composed — an aggregate can wrap a function of a board function, etc. See [Built-in Functions](../functions/overview.md) for the list of operators you can combine.
