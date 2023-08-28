use std::default;
use std::error;
use std::fmt;
use std::ops::Range;
use std::ops::Index;

type Value = u8;

/// The set of errors that can occur in this application
#[derive(Debug)]
pub enum Error {
    IdError{ admissible: Range<usize>, actual: usize },
    ValueError{ value: Value, expected: String },
    ConstraintError{ region: String, slice: Slice }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ConstraintError { region, slice } => {
                write!(f, "expected numbers 1..9 in {region} but got values {slice}")
            },
            Error::ValueError { value, expected } => {
                write!(f, "expected {expected} as value but got {value}")
            },
            Error::IdError { admissible, actual } => {
                write!(f, "expected valid ID in range {}..{} but got {}", admissible.start, admissible.end, actual)
            },
        }
    }
}

/// `Cell` is a wrapper for `u8`. Its only purpose is
/// to provide convenient string representations for the
/// content of a cell.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Cell(Value);

impl Eq for Cell {}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if 1 <= self.0 && self.0 <= 9 {
            write!(f, "{:^3}", self.0)
        } else {
            // NOTE: "I" as in "invalid"
            write!(f, " I ")
        }
    }
}

/// `Slice` is a set of 9 cells. Sudoku often operates on 9 cell elements.
/// A `Slice` is the result if you access a column, row, or block by some index.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Slice([Cell; 9]);

impl Slice {
    pub fn set(&mut self, index: usize, cell: Cell) {
        self.0[index] = cell;
    }

    /// Does this slice contain the provided `Value`?
    pub fn has(&self, value: Value) -> bool {
        for i in 0..9 {
            if self.0[i].0 == value {
                return true;
            }
        }
        false
    }

    /// Are the admissible Sudoku values inside the cells unique?
    pub fn has_unique_sudoku_values(&self) -> bool {
        let mut count = [0; 9];
        for cell in self.0.iter() {
            // NOTE: consider only admissible values
            if 1 <= cell.0 && cell.0 <= 9 {
                count[cell.0 as usize - 1] += 1;
            }
        }
        for occurences in count.iter() {
            if *occurences > 1 {
                return false;
            }
        }

        true
    }

    /// Which Sudoku values are unused in this `Slice`?
    pub fn unused_sudoku_values(&self) -> Vec<Value> {
        let mut unused = vec![];

        for candidate in 1..=9 {
            let mut found = false;
            for i in 0..9 {
                if self.0[i].0 == candidate {
                    found = true;
                    break;
                }
            }

            if !found {
                unused.push(candidate);
            }
        }

        unused
    }
}

impl default::Default for Slice {
    fn default() -> Self {
        Self([Cell(0); 9])
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().map(|cell| format!("{}", cell)).collect::<Vec<String>>().join(""))
    }
}

/// The sudoku board containing 81 `Cell`s.
/// Each cell is identified by some index or its row & column tuple.
/// The `Cell` store the value (0 means unassigned, 1..=9 are Sudoku values)
/// 
/// All operations on this board are unchecked which is why I don't expose
/// them beyond crate boundaries. And within the crate, use them with care!
#[derive(Clone,Debug)]
pub struct Board {
    cells: [Cell; Self::COUNT_ROWS * Self::COUNT_COLUMNS],
}

impl Board {
    const COUNT_VALUES: usize = 9 * 9;
    const COUNT_BLOCKS: usize = 9;
    const COUNT_ROWS: usize = 9;
    const COUNT_COLUMNS: usize = 9;

    /// Update the board's entries using the values provided.
    /// All values are provided in one long linear array
    /// from top-left to top-right until the last row and finally bottom-right.
    pub(crate) fn from_flattened_values(values: &[Value; Self::COUNT_VALUES]) -> Self {
        let mut new_cells = [Cell(0); Self::COUNT_VALUES];
        for i in 0..Self::COUNT_VALUES {
            new_cells[i] = Cell(values[i]);
        }
        Self { cells: new_cells }
    }

    /// Update the board's entries with the values provided per row in one array.
    /// Specifically, there are as many arrays as there are rows on the board.
    /// And there are as many entries per row as there are columns.
    pub(crate) fn from_values_per_row(values: &[[Value; Self::COUNT_COLUMNS]; Self::COUNT_ROWS]) -> Self {
        let mut new_cells = [Cell(0); Self::COUNT_VALUES];
        for row_id in 0..Self::COUNT_ROWS {
            for column_id in 0..Self::COUNT_COLUMNS {
                new_cells[row_id * Self::COUNT_COLUMNS + column_id] = Cell(values[row_id][column_id]);
            }
        }
        Self { cells: new_cells }
    }

    /// Return the cell given its zero-based row and column number
    pub(crate) fn index_by_row_and_col(&self, row: usize, col: usize) -> Cell {
        self[row * Self::COUNT_COLUMNS + col]
    }

    /// Return the set of indices of unassigned values
    pub(crate) fn unassigned(&self) -> Vec<usize> {
        let mut unassigned = vec![];
        for cell_id in 0..Board::COUNT_VALUES {
            // ASSUME: cells with value "0" are "unassigned"
            if self[cell_id].0 == 0 {
                unassigned.push(cell_id);
            }
        }
        unassigned
    }

    /// Return the cells of a block (9×9) given an identifier from 0 to 8.
    /// 0 is at the top-left, 2 is at the top-right, 8 is at the bottom-right.
    pub(crate) fn block(&self, block_id: usize) -> Slice {
        let base_cell_id_per_block = [0, 3, 6, 27, 30, 33, 54, 57, 60];
        let mut block = Slice::default();

        let base: usize = base_cell_id_per_block[block_id];
        let mut i = 0;
        for row_offset in [0, 9, 18] {
            for col_offset in [0, 1, 2] {
                block.set(i, self.cells[base + row_offset + col_offset]);
                i += 1;
            }
        }

        block
    }

    /// Return the cells of a row given a row identifier from 0 to 8.
    pub(crate) fn row(&self, row_id: usize) -> Slice {
        let mut row = Slice::default();
        for column_id in 0..9 {
            row.set(column_id, self.cells[row_id * 9 + column_id]);
        }
        row
    }

    /// Return the cells of a column given a column identifier from 0 to 8.
    pub(crate) fn column(&self, column_id: usize) -> Slice {
        let mut column = Slice::default();
        for row_id in 0..9 {
            column.set(row_id, self.cells[row_id * 9 + column_id]);
        }
        column
    }

    /// Replace one value of the board and return the updated `Board` instance
    pub(crate) fn replace_cell(&self, cell_id: usize, value: Value) -> Board {
        let mut b = self.clone();
        b.cells[cell_id] = Cell(value);
        b
    }

    /// String representation of the `Board`, but highlight the cell at the given index
    pub(crate) fn to_highlighted_string(&self, highlighted_cell: usize) -> String {
        let mut out = format!("┌{}┐\n", "─".repeat(27));

        for row_id in 0..Self::COUNT_ROWS {
            out.push('│');
            for column_id in 0..Self::COUNT_COLUMNS {
                let cell_id = Self::COUNT_COLUMNS * row_id + column_id;
                let cell = self.cells[cell_id];
                if cell_id == highlighted_cell {
                    out.push_str(&format!("\x1B[0;33m{}\x1B[0;39m", cell));
                } else {
                    out.push_str(&format!("{}", cell));
                }
            }
            out.push_str("│\n");
        }

        out.push_str(&format!("└{}┘", "─".repeat(27)));
        out
    }
}

impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, id: usize) -> &Self::Output {
        &self.cells[id]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self { cells: [Cell(0); 9 * 9] }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌{}┐", "─".repeat(27))?;

        for row_id in 0..9 {
            write!(f, "│")?;
            write!(f, "{}", self.cells[9 * row_id..9 * row_id + 9].iter().map(|cell| (*cell).to_string()).collect::<Vec<String>>().join(""))?;
            writeln!(f, "│")?;
        }

        writeln!(f, "└{}┘", "─".repeat(27))
    }
}

/// The game instance of Sudoku. So it contains a board as current state
/// and can be extended by further game-related members.
#[derive(Clone,Debug,Default)]
pub struct Sudoku {
    board: Board,
}

impl Sudoku {
    pub fn init_board(&mut self, board: &Board) {
        self.board = board.clone();
    }

    pub fn init_board_values(&mut self, values: &[Value; Board::COUNT_VALUES]) {
        self.board = Board::from_flattened_values(values);
    }

    /// Reference to the Board instance active in this game
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Does our board satisfy all Sudoku constraints?
    /// If yes, returns nothing. If no, returns a ``Error::ConstraintError``.
    pub fn verify_board(&self) -> Result<(), Error> {
        for column_id in 0..Board::COUNT_COLUMNS {
            let col = self.board.column(column_id);
            if !col.has_unique_sudoku_values() {
                return Err(Error::ConstraintError { region: format!("column {}", column_id + 1), slice: col });
            }
        }

        for row_id in 0..Board::COUNT_ROWS {
            let row = self.board.row(row_id);
            if !row.has_unique_sudoku_values() {
                return Err(Error::ConstraintError { region: format!("row {}", row_id + 1), slice: row });
            }
        }

        for block_id in 0..Board::COUNT_BLOCKS {
            let block = self.board.block(block_id);
            if !block.has_unique_sudoku_values() {
                let vertical_pos = ["top", "middle", "bottom"];
                let horizontal_pos = ["left", "center", "right"];
                return Err(Error::ConstraintError { region: format!("block {}-{}", vertical_pos[block_id / 3], horizontal_pos[block_id % 3]), slice: block });
            }
        }

        Ok(())
    }

    /// Is the game finished in this state?
    pub fn finished(&self) -> bool {
        self.board().unassigned().is_empty()
    }

    /// Determine the set of next possible moves.
    /// Returns a list of tuples containing the cell ID which changed and the updated Board instance.
    pub fn next_possible_moves(&self) -> Vec<(usize, Board)> {
        let b = self.board();
        let cells_to_update = b.unassigned();

        let mut moves = vec![];
        for cell_id in cells_to_update {
            for candidate_value in 1..=9 {
                let column_id = cell_id % 9;
                let row_id = cell_id / 9;

                let col = b.column(column_id);
                if col.has(candidate_value as Value) {
                    continue;
                }

                let row = b.row(row_id);
                if row.has(candidate_value as Value) {
                    continue;
                }

                let block = b.block(row_id);
                if block.has(candidate_value as Value) {
                    continue;
                }

                moves.push((cell_id, b.replace_cell(cell_id, candidate_value)));
            }
        }

        moves
    }
}


fn main() -> Result<(), Error> {
    let example_values = [
        0, 0, 0, 2, 6, 0, 7, 0, 1,
        6, 8, 0, 0, 7, 0, 0, 9, 0,
        1, 9, 0, 0, 0, 4, 5, 0, 0,
        8, 2, 0, 1, 0, 0, 0, 4, 0,
        0, 0, 4, 6, 0, 2, 9, 0, 0,
        0, 5, 0, 0, 0, 3, 0, 2, 8,
        0, 0, 9, 3, 0, 0, 0, 7, 4,
        0, 4, 0, 0, 5, 0, 0, 3, 6,
        7, 0, 3, 0, 1, 8, 0, 0, 0,
    ];

    let mut sudoku = Sudoku::default();
    sudoku.init_board_values(&example_values);

    println!("{}", sudoku.board());
    println!("this sudoku game has{} reached its end", if sudoku.finished() { "" } else { "NOT yet " });

    sudoku.verify_board()?;

    let mut count_solutions = 0;
    for (updated_cell_id, updated_board) in sudoku.next_possible_moves() {
        let new_value = updated_board[updated_cell_id];
        let (row_id, col_id) = (updated_cell_id / 9, updated_cell_id % 9);
        println!("Next possible move:  set row {} column {} to {}", row_id + 1, col_id + 1, new_value.0);
        println!("{}", updated_board.to_highlighted_string(updated_cell_id));
        count_solutions += 1;
    }
    println!("there are {} solutions to move on", count_solutions);

    Ok(())
}
