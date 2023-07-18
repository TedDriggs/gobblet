use std::fmt;

use crate::{Board, Line, Move, Player};

/// The number of pieces each player has of each size at the start
/// of the game.
const STARTING_INVENTORY: usize = 2;

/// A game instance, which includes all moves up to the current point
/// in the game.
#[derive(Default)]
pub struct Game {
    board: Board,
    moves: Vec<Move>,
    victory: Option<Victory>,
}

impl Game {
    /// Get the player whose turn it is.
    ///
    /// # Example
    /// ```
    /// # use gobblet::{Cell, Game, Move, Player, Size};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut game = Game::default();
    /// assert_eq!(game.next_player(), Player::One);
    ///
    /// game.submit(Move::new(Player::One, Size::Small, None, Cell::new(0, 0)?))?;
    /// assert_eq!(game.next_player(), Player::Two);
    /// # Ok(()) }
    pub fn next_player(&self) -> Player {
        self.moves.last().map(|m| !m.player).unwrap_or(Player::One)
    }

    /// Submit a move to the game.
    ///
    /// # Example
    /// ```
    /// # use gobblet::{Cell, Game, Move, Player, Size};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut game = Game::default();
    /// let victory = game.submit(Move::new(Player::One, Size::Small, None, Cell::new(0, 0)?))?;
    ///
    /// assert!(victory.is_none());
    /// assert_eq!(game.moves().len(), 1);
    /// assert_eq!(game.next_player(), Player::Two);
    /// # Ok(()) }
    pub fn submit(&mut self, mv: Move) -> Result<Option<Victory>, SubmitMoveError> {
        if self.victory.is_some() {
            return Err(SubmitMoveError::GameOver);
        }

        if self.next_player() != mv.player {
            return Err(SubmitMoveError::OutOfTurn);
        }

        if let Some(source) = mv.source {
            let source_state = &self.board[source];
            if source_state[mv.size] != Some(mv.player) {
                return Err(SubmitMoveError::PieceNotAtSource);
            }

            if source_state
                .size()
                .map(|src_size| src_size > mv.size)
                .unwrap_or(false)
            {
                return Err(SubmitMoveError::PieceBlockedAtSource);
            }
        } else {
            let in_play = self
                .board
                .cells()
                .filter(|(_, state)| state[mv.size] == Some(mv.player))
                .count();

            if in_play >= STARTING_INVENTORY {
                return Err(SubmitMoveError::PieceNotInInventory);
            }
        };

        if self.board[mv.target]
            .size()
            .map(|dest_size| dest_size >= mv.size)
            .unwrap_or(false)
        {
            return Err(SubmitMoveError::TargetBlocked);
        }

        mv.source.map(|src| self.board[src][mv.size] = None);
        self.board[mv.target][mv.size] = Some(mv.player);

        let victory = look_for_victory(&self.board, mv.player);

        self.moves.push(mv);

        self.victory = victory;
        Ok(victory)
    }

    /// Get the current state of the board.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get the moves in order of submission.
    pub fn moves(&self) -> &[Move] {
        &self.moves
    }

    /// Get the outcome of the game; if this is `None`, the game has not ended.
    pub fn outcome(&self) -> Option<Victory> {
        self.victory
    }
}

/// Look for a victory state in a given board.
///
/// The rules of the game say that if a player's move results in both players
/// having a completed line, the player who just moved loses. This avoids a
/// situation in which player one wins by uncovering a complete line for player
/// two and completing their own line at the same time.
fn look_for_victory(board: &Board, last_moving_player: Player) -> Option<Victory> {
    let mut win_for_last_moving_player = None;

    let first_choice_winner = !last_moving_player;

    for line in Line::all() {
        let Some(winner) = board
            .line(line)
            .map(|(_, state)| state.controlled_by())
            .reduce(|acc, curr| if acc == curr { acc } else { None })
            .flatten() else { continue; };

        if winner == first_choice_winner {
            return Some(Victory {
                player: first_choice_winner,
                line,
            });
        } else {
            if win_for_last_moving_player.is_none() {
                win_for_last_moving_player = Some(line);
            }
        }
    }

    // If there is no victory for the first-choice winner, then the
    // victory will be that of the last-moving player - or None, if
    // the last-moving player also doesn't have a victory.
    win_for_last_moving_player.map(|line| Victory {
        player: last_moving_player,
        line,
    })
}

/// The terminal state of a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Victory {
    player: Player,
    line: Line,
}

impl Victory {
    /// The player who won the game.
    pub fn player(&self) -> Player {
        self.player
    }

    /// The line that won the game for the player.
    ///
    /// In the event that the board has multiple winning lines for
    /// the player, only one will be returned. This is stable, but implementation-defined.
    pub fn line(&self) -> Line {
        self.line
    }
}

impl fmt::Display for Victory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} wins on {}", self.player, self.line)
    }
}

/// An error that prevented evaluation of a submitted move.
///
/// See [`Game::submit`].
#[derive(Debug, thiserror::Error)]
pub enum SubmitMoveError {
    #[error("The game has already ended")]
    GameOver,
    #[error("Other player's turn")]
    OutOfTurn,
    #[error("Piece is not present at source")]
    PieceNotAtSource,
    #[error("Piece not available to be moved from inventory")]
    PieceNotInInventory,
    #[error("Piece is present at source, but is blocked by a larger piece")]
    PieceBlockedAtSource,
    #[error("A piece of the same or greater size is already at the destination")]
    TargetBlocked,
}
