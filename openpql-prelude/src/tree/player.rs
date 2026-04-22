use derive_more::Display;

pub use crate::PlayerIdx;

/// Number of players at the table.
pub type PlayerCount = u8;

/// Game-tree node actor: chance, a seated player, or terminal.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Display, Default, Hash, PartialEq, Eq, derive_more::From,
)]
pub enum Player {
    /// Dealer at a chance node.
    #[default]
    Chance,
    /// Seated player identified by index.
    Player(PlayerIdx),
    /// Terminal leaf with no acting player.
    Terminal,
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::Player;
    use crate::*;

    #[test]
    fn test_player_chance_ser_de() {
        assert_tokens(
            &Player::Chance,
            &[Token::UnitVariant {
                name: "Player",
                variant: "Chance",
            }],
        );
    }

    #[test]
    fn test_player_player_ser_de() {
        assert_tokens(
            &Player::Player(5),
            &[
                Token::NewtypeVariant {
                    name: "Player",
                    variant: "Player",
                },
                Token::U8(5),
            ],
        );
    }

    #[test]
    fn test_player_terminal_ser_de() {
        assert_tokens(
            &Player::Terminal,
            &[Token::UnitVariant {
                name: "Player",
                variant: "Terminal",
            }],
        );
    }
}
