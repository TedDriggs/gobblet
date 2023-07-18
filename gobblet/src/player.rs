use std::{fmt, ops::Not};

/// An agent which can submit moves in the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player {
    /// The first player to move in the game.
    One,
    /// The second player to move in the game.
    Two,
}

impl Player {
    fn player_number(&self) -> &'static str {
        match self {
            Player::One => "1",
            Player::Two => "2",
        }
    }
}

/// Get the other player.
impl Not for Player {
    type Output = Player;

    fn not(self) -> Self::Output {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if f.alternate() { "P" } else { "Player " },
            self.player_number()
        )
    }
}
