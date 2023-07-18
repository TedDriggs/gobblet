use std::fmt;
use std::str::FromStr;

use crate::Cell;

use crate::Size;

use crate::board::CellError;
use crate::Player;

/// A player's move of a piece.
#[derive(Debug, Clone)]
pub struct Move {
    /// The player moving a piece.
    pub player: Player,
    /// The size of the piece being moved.
    pub size: Size,
    /// Where the moved piece was at the start of the turn. If `None`, the piece
    /// comes from the player's off-board inventory.
    pub source: Option<Cell>,
    /// Where the moved piece goes.
    pub target: Cell,
}

impl Move {
    /// Create a new instance of `Self`.
    pub fn new(player: Player, size: Size, source: Option<Cell>, target: Cell) -> Self {
        Self {
            player,
            size,
            source,
            target,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ", self.player, self.size)?;
        if let Some(source) = self.source {
            write!(f, "{}", source)?;
        } else {
            write!(f, "Inventory")?;
        }
        write!(f, " > {}", self.target)
    }
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        let num_parts = parts.len();
        if num_parts < 5 {
            return Err(ParseMoveError::TooFewParts(num_parts));
        } else if num_parts > 5 {
            return Err(ParseMoveError::TooManyParts(num_parts));
        }

        let player = parse_player(parts[0])?;
        let size = parse_size(parts[1])?;
        let source = parse_source(parts[2]).map_err(ParseMoveError::InvalidSource)?;
        let _arrow = parts[3];
        let target = parse_cell(parts[4]).map_err(ParseMoveError::InvalidTarget)?;

        Ok(Self {
            player,
            size,
            source,
            target,
        })
    }
}

/// Error encountered when parsing a [`Move`] from its string representation.
#[derive(Debug, thiserror::Error)]
pub enum ParseMoveError {
    #[error("Too few parts")]
    TooFewParts(usize),
    #[error("Too many parts")]
    TooManyParts(usize),
    #[error("Invalid player")]
    InvalidPlayer,
    #[error("Invalid size")]
    InvalidSize,
    #[error("Invalid source")]
    InvalidSource(#[source] ParseCellError),
    #[error("Invalid target")]
    InvalidTarget(#[source] ParseCellError),
}

fn parse_player(p: &str) -> Result<Player, ParseMoveError> {
    match p {
        "P1" | "p1" => Ok(Player::One),
        "P2" | "p2" => Ok(Player::Two),
        _ => Err(ParseMoveError::InvalidPlayer),
    }
}

fn parse_size(size: &str) -> Result<Size, ParseMoveError> {
    match size {
        "S" | "s" => Ok(Size::Small),
        "M" | "m" => Ok(Size::Medium),
        "L" | "l" => Ok(Size::Large),
        _ => Err(ParseMoveError::InvalidSize),
    }
}

fn parse_source(source: &str) -> Result<Option<Cell>, ParseCellError> {
    if source == "_" {
        Ok(None)
    } else {
        parse_cell(source).map(Some)
    }
}

fn parse_cell(cell: &str) -> Result<Cell, ParseCellError> {
    let Some((row, col)) = cell.split_once(",") else {
        return Err(ParseCellError::TooFewParts);
    };
    let Ok(row) = row.parse() else {
        return Err(ParseCellError::InvalidRow);
    };

    let Ok(col) = col.parse() else {
        return Err(ParseCellError::InvalidColumn);
    };

    Ok(Cell::new(row, col)?)
}

/// Error encountered when parsing a [`Cell`] as part of a [`Move`].
#[derive(Debug, thiserror::Error)]
pub enum ParseCellError {
    #[error("Too few parts: A cell should be row,col")]
    TooFewParts,
    #[error("Invalid row")]
    InvalidRow,
    #[error("Invalid column")]
    InvalidColumn,
    #[error("Out of bounds")]
    OutOfBounds(#[from] CellError),
}
