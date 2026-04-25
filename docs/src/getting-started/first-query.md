# Your First Query

Let's compute hero's equity against a range of villain hands on a known flop.

## The Query

```sql
select equity(hero)
from   game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'
```

Reading left to right:

- **`select equity(hero)`** — ask for hero's equity (sampled per trial).
- **`game='holdem'`** — play the hand as Texas Hold'em.
- **`hero='AhKh'`** — hero holds the Ace and King of hearts (specific cards).
- **`villain='QQ+'`** — villain has any pocket pair QQ or better.
- **`board='Ah9s2c'`** — three community cards are already on the table.

## Run It

```bash
opql --run "select equity(hero) from game='holdem', hero='AhKh', villain='QQ+', board='Ah9s2c'"
```

The runner samples random runouts (turn + river) and prints the resulting equity. Re-run the query for a fresh Monte Carlo estimate.

## A Second Example

Average number of suits on the river when hero holds A♠9♠ on a two-tone flop:

```bash
opql --run "select avg(boardSuitCount(river)) from game='holdem', hero='As9s', villain='*', board='2s3sJh'"
```

Here `villain='*'` means "any two cards", and `avg(...)` averages a per-trial value across samples.

## A Counting Example

Frequency hero flops top pair against any two cards:

```bash
opql --run "select count(hasTopBoardRank(hero, flop)) as pct_toppair from game='holdem', hero='AxKy', villain='*', board=''"
```

`count(predicate)` divides the number of trials where the predicate held by the total number of trials, giving you a probability.

## Next Step

Learn more about CLI usage in [CLI Basics](./cli.md), or skip ahead to [Query Structure](../language/query-structure.md).
