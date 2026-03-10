pub const WIDTH: usize = 15;
pub const HEIGHT: usize = 15;
pub const TOTAL_CELLS: usize = WIDTH * HEIGHT;
pub const TOTAL_MINES: usize = 25;

#[derive(Clone, Copy, PartialEq)]
pub enum CellContent {
    Empty(u8),
    Mine,
}
#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Flagged,
    Revealed,
    Hidden,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub content: CellContent,
    pub state: CellState,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: CellContent::Empty(0),
            state: CellState::Hidden,
        }
    }
}
