# Your First Query

Let's compute hero's equity against a range of villain hands.

## The Query

```sql
select equity
from   hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'
```

Reading left to right:

- **`select equity`** — ask for the equity selector
- **`hero='AhKh'`** — hero holds the Ace and King of hearts
- **`villain='QQ+'`** — villain has any pocket pair QQ or better
- **`board='Ah9s2c'`** — the community cards on the flop
- **`game='holdem'`** — play the hand as Texas Hold'em

## Run It

```bash
opql --run "select equity from hero='AhKh', villain='QQ+', board='Ah9s2c', game='holdem'"
```

The runner samples random runouts (turn + river) and prints the resulting equity. Re-run the query for a fresh Monte Carlo estimate.

## A Second Example

Average number of suited cards on the river when hero holds A♠9♠ and the flop already has two spades:

```bash
opql --run "select avg(boardsuitcount(river)) from hero='As9s', villain='*', board='2s3sJh', game='holdem'"
```

Here `villain='*'` means any two cards, and `avg(...)` averages a per-trial statistic across samples.

## Next Step

Learn more about the CLI flags in [CLI Basics](./cli.md), or skip ahead to [Query Structure](../language/query-structure.md).
