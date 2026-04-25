# Postflop Analysis

Once you've fixed a flop (or turn or river) you can ask more specific questions — frequencies of made hands, equity by street, conditional probabilities, and so on.

## Flopping a Set with a Pocket Pair

How often does hero flop a set with pocket tens?

```sql
select count(flopHandCategory(hero) = flopset) as pct_set
from   game='holdem', hero='TT', villain='*', board=''
```

`board=''` means we sample every flop. `count(predicate)` divides the trials where the predicate held by the total trial count, giving a probability.

## Equity Holding a Flush Draw

Hero has a nut flush draw on the flop; what's the showdown equity against an over-pair range?

```sql
select avg(equity(hero))
from   game='holdem', hero='AhKh', villain='AA-JJ', board='Qh7h2c'
```

## Conditional: Pair on the Turn Given Two Overcards

Given hero holds two unpaired overcards on a low flop, how often does the turn pair one of them?

```sql
select count(hasTopBoardRank(hero, turn)) as pct_pair_on_turn
from   game='holdem', hero='AxKy', villain='*', board='8s5d2c'
```

## Equity on a Specific River Texture

Average river equity for an overpair against a random hand, restricted to non-paired rivers:

```sql
select avg(riverEquity(hero))
from   game='holdem', hero='QQ', villain='**', board='Jc7d2s'
where  not pairedBoard(river)
```

## Multiple Selectors at Once

Mix probabilities, equities, and counts in a single query:

```sql
select count(overpair(hero, river))                as pct_overpair_held,
       count(handType(hero, river) = twopair)     as pct_twopair,
       avg(riverEquity(hero))                     as avg_eq
from   game='holdem', hero='QQ', villain='**', board='Jc7d2s'
```
