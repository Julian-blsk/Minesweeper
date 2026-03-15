pub const WIDTH: usize = 15;
pub const HEIGHT: usize = 15;
pub const TOTAL_CELLS: usize = WIDTH * HEIGHT;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CellContent {
    Empty(u8),
    Mine,
}

impl Default for CellContent {
    fn default() -> Self {
        CellContent::Empty(0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Hidden
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub content: CellContent,
    pub state: CellState,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: CellContent::default(),
            state: CellState::default(),
        }
    }
}
