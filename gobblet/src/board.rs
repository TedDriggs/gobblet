use std::{
    fmt, iter,
    ops::{Index, IndexMut},
};

use crate::{Player, Size};

/// Coordinates for a cell on a [`Board`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell(usize);

impl Cell {
    /// Create a new instance of `Self`.
    ///
    /// # Errors
    /// This function returns an error if the row or column is out of bounds
    /// for a standard board.
    pub fn new(row: u8, col: u8) -> Result<Self, CellError> {
        if row > 2 {
            return Err(CellError::RowOutOfBounds);
        }

        if col > 2 {
            return Err(CellError::ColumnOutOfBounds);
        }

        Ok(Self((row * 3 + col).into()))
    }

    /// The 0-indexed row of the cell.
    pub fn row(&self) -> u8 {
        (self.0 / 3).try_into().unwrap()
    }

    /// The 0-indexed column of the cell.
    pub fn col(&self) -> u8 {
        (self.0 % 3).try_into().unwrap()
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}c{}", self.row(), self.col())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}c{}", self.row(), self.col())
    }
}

/// Error encountered during creation of a cell.
///
/// See [`Cell::new`].
#[derive(Debug, thiserror::Error)]
pub enum CellError {
    #[error("Row out of bounds")]
    RowOutOfBounds,
    #[error("Column out of bounds")]
    ColumnOutOfBounds,
}

/// The contents of a [`Cell`].
#[derive(Default)]
pub struct CellState {
    small: Option<Player>,
    medium: Option<Player>,
    large: Option<Player>,
}

impl CellState {
    /// Get the player who currently controls the cell.
    pub fn controlled_by(&self) -> Option<Player> {
        self.large.or(self.medium).or(self.small)
    }

    /// Get the size of the largest piece in the cell.
    pub fn size(&self) -> Option<Size> {
        if self.large.is_some() {
            Some(Size::Large)
        } else if self.medium.is_some() {
            Some(Size::Medium)
        } else if self.small.is_some() {
            Some(Size::Small)
        } else {
            None
        }
    }
}

impl Index<Size> for CellState {
    type Output = Option<Player>;

    fn index(&self, index: Size) -> &Self::Output {
        match index {
            Size::Small => &self.small,
            Size::Medium => &self.medium,
            Size::Large => &self.large,
        }
    }
}

impl IndexMut<Size> for CellState {
    fn index_mut(&mut self, index: Size) -> &mut Self::Output {
        match index {
            Size::Small => &mut self.small,
            Size::Medium => &mut self.medium,
            Size::Large => &mut self.large,
        }
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(controlled_by) = self.controlled_by() else {
            return write!(f, "    ");
        };

        write!(
            f,
            "{:#}-{:#}",
            controlled_by,
            self.size()
                .expect("Cell needs to be occupied to be controlled")
        )
    }
}

/// A point-in-time game board.
#[derive(Default)]
pub struct Board {
    cells: [CellState; 9],
}

impl Board {
    /// Iterate over each cell's coordinates and current state.
    ///
    /// Iteration order is implementation-defined.
    pub fn cells(&self) -> impl Iterator<Item = (Cell, &CellState)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, item)| (Cell(idx), item))
    }

    /// Get cell coordinates and state for the three cells on the specified line.
    pub fn line(&self, line: Line) -> impl Iterator<Item = (Cell, &CellState)> {
        self.cells().filter(move |(c, _)| line.matches(c))
    }
}

impl Index<Cell> for Board {
    type Output = CellState;

    fn index(&self, index: Cell) -> &Self::Output {
        &self.cells[index.0]
    }
}

impl IndexMut<Cell> for Board {
    fn index_mut(&mut self, index: Cell) -> &mut Self::Output {
        &mut self.cells[index.0]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (Cell(offset), state) in self.cells() {
            if offset % 3 > 0 {
                write!(f, "|")?;
            } else if offset > 0 {
                writeln!(f, "--------------")?;
            }

            write!(f, "{}", state)?;

            if offset % 3 == 2 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

/// Identifier of a row, column, or diagonal which can be used to match cells in a [`Board`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line {
    Row(u8),
    Col(u8),
    DiagonalUp,
    DiagonalDown,
}

impl Line {
    /// Check if a cell coordinate is on this line.
    ///
    /// # Example
    /// ```
    /// # use gobblet::{Cell, Line};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let line = Line::Row(0);
    /// assert!(line.matches(&Cell::new(0, 0)?));
    /// # Ok(()) }
    /// ```
    pub fn matches(&self, cell: &Cell) -> bool {
        match self {
            Line::Row(r) => cell.row() == *r,
            Line::Col(c) => cell.col() == *c,
            Line::DiagonalUp => cell.row() + cell.col() == 2,
            Line::DiagonalDown => cell.row() == cell.col(),
        }
    }

    /// Get all possible lines for the board.
    ///
    /// Iteration order is implementation-defined.
    pub fn all() -> impl Iterator<Item = Line> {
        (0..3)
            .map(Self::Row)
            .chain((0..3).map(Self::Col))
            .chain(iter::once(Self::DiagonalUp))
            .chain(iter::once(Self::DiagonalDown))
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Line::Row(r) => write!(f, "row {}", r + 1),
            Line::Col(c) => write!(f, "column {}", c + 1),
            Line::DiagonalUp => write!(f, "diagonal ↗"),
            Line::DiagonalDown => write!(f, "diagonal ↘"),
        }
    }
}
