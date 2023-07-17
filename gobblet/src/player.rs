use std::{fmt, ops::Not};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player {
    One,
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
