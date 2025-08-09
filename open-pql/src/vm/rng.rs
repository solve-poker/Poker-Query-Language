use super::*;

#[derive(Debug, Clone)]
pub struct Rng {
    initial_state: Vec<Card>,
    current_state: Vec<Card>,
    pub(crate) mem: Vec<Card>,
    mem_unused: Vec<Card>,
}

impl Default for Rng {
    fn default() -> Self {
        Self::new(Card::ARR_ALL.to_vec())
    }
}

impl Rng {
    pub fn new(initial: Vec<Card>) -> Self {
        let len = initial.len();

        Self {
            initial_state: initial,
            current_state: Vec::with_capacity(len),
            mem: Vec::with_capacity(5),
            mem_unused: Vec::with_capacity(len),
        }
    }

    pub fn reset(&mut self) {
        self.current_state.clone_from(&self.initial_state);
    }

    pub fn deal_n(&mut self, range: &mut PQLRange, mut n: u8) -> Option<()> {
        while n > 0 {
            let len = self.current_state.len();

            if len > 0 {
                let i = fastrand::usize(0..len);
                let c = self.current_state.swap_remove(i);

                self.mem.push(c);

                if range.is_satisfied(self.mem.as_ref()) {
                    n -= 1;
                } else {
                    self.mem.pop();
                    self.mem_unused.push(c);
                }
            } else {
                return None;
            }
        }

        self.current_state.append(&mut self.mem_unused);

        Some(())
    }

    pub fn deal(&mut self, range: &mut PQLRange) -> Option<()> {
        self.mem_unused.clear();
        self.mem.clear();

        match range {
            PQLRange::Hand2(_) => self.deal_n(range, 2),
            PQLRange::Hand4(_) => self.deal_n(range, 4),
            PQLRange::Board(_) => {
                self.deal_n(range, 3)?;
                self.deal_n(range, 1)?;
                self.deal_n(range, 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;
    use crate::{Card, Card64, c64, cards, *};

    #[test]
    fn test_reset() {
        let cards = cards!["As 2h"];

        let mut dealer = Rng::new(cards.clone());
        dealer.current_state.clear();

        dealer.reset();

        assert_eq!(dealer.current_state, cards);
    }

    #[test]
    fn test_deal() {
        fastrand::seed(0);
        let mut range = PQLRange::from_src("AA", PQLGame::Omaha).unwrap();

        let mut dealer = Rng::new(Card::ARR_ALL.to_vec());
        dealer.reset();

        let res = dealer.deal(&mut range);
        let c = c64!("As 2s 9c Ac");

        assert!(res.is_some());
        assert_eq!(Card64::from(dealer.mem.as_ref()), c);
        assert_eq!(
            Card64::from(dealer.current_state.as_ref()),
            Card64::all() & !c
        );

        dealer.mem.clear();
        let res = dealer.deal_n(&mut range, 2);
        let c_n = c64!("6s Td");

        assert!(res.is_some());
        assert_eq!(Card64::from(dealer.mem.as_ref()), c_n);
        assert_eq!(
            Card64::from(dealer.current_state.as_ref()),
            Card64::all() & !c & !c_n
        );

        dealer.current_state.clear();
        assert_eq!(dealer.deal(&mut range), None);

        let mut dealer = Rng::new(cards!("2s 2h 2d 2c 3s"));

        dealer.reset();
        assert!(
            dealer
                .deal(&mut PQLBoardRange::from_src("AAA").unwrap().into())
                .is_none()
        );
        dealer.reset();
        assert!(
            dealer
                .deal(&mut PQLBoardRange::from_src("222A").unwrap().into())
                .is_none()
        );
    }
}
