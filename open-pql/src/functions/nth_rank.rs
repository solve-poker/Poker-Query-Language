use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn nth_rank(n: PQLCardCount, ranks: PQLRankSet) -> Option<PQLRank> {
    ranks.nth_rank(n.to_le_bytes()[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_nth_rank(cs: Vec<Card>) {
        const N: usize = 8;
        let cs = cs.into_iter().take(N);
        let rs = cs.map(|c| c.rank).collect::<Vec<_>>();

        let ranks = Rank16::from(rs.as_ref() as &[Rank]);
        let mut sorted = rs;

        sorted.sort();
        sorted.dedup();
        sorted.reverse();

        for (i, r) in sorted.into_iter().enumerate() {
            assert_eq!(Some(r), nth_rank((i + 1).try_into().unwrap(), ranks));
        }
    }
}
