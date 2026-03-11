use crate::board::Board;
use crate::cell::{Cell, CellContent, CellState, HEIGHT, TOTAL_CELLS, TOTAL_MINES, WIDTH};
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Won,
    Lost,
    Running,
}

#[derive(Clone)]
pub struct Game {
    pub state: GameState,
    pub board: Board,
    pub mines_left: usize,
    pub mines_placed: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Running,
            board: Board {
                grid: [Cell::default(); TOTAL_CELLS],
            },
            mines_left: TOTAL_MINES,
            mines_placed: false,
        }
    }

    pub fn place_mines(&mut self, first_x: usize, first_y: usize) {
        let mut rng = rand::thread_rng();
        let mut placed = 0;

        while placed < TOTAL_MINES {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);

            if x == first_x && y == first_y {
                continue;
            }

            if let Some(cell) = self.board.get_mut_cell(x, y) {
                if cell.content != CellContent::Mine {
                    cell.content = CellContent::Mine;
                    placed += 1;
                }
            }
        }
        self.mines_placed = true;
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) -> bool {
        if !self.mines_placed {
            self.place_mines(x, y);
        }
        {
            let cell = match self.board.get_mut_cell(x, y) {
                Some(c) => c,
                None => return true,
            };
            if cell.state == CellState::Revealed || cell.state == CellState::Flagged {
                return true;
            }
            if cell.content == CellContent::Mine {
                self.state = GameState::Lost;
                cell.state = CellState::Revealed;
                return false;
            }
        }
        let mine_count = self.board.count_neighbor_mines(x, y);
        {
            let cell = match self.board.get_mut_cell(x, y) {
                Some(c) => c,
                None => return true,
            };
            cell.content = CellContent::Empty(mine_count);
            cell.state = CellState::Revealed;
            if mine_count as i32 == 0 {
                self.reveal_neighbors(x, y);
            }
            if self.check_win_condition() {
                self.state = GameState::Won;
                return false;
            }
        }
        true
    }
    pub fn check_win_condition(&self) -> bool {
        for i in 0..TOTAL_CELLS {
            let cell = self.board.grid[i];
            if cell.content != CellContent::Mine && cell.state != CellState::Revealed {
                return false;
            }
        }
        return true;
    }
    pub fn reveal_neighbors(&mut self, x: usize, y: usize) {
        let neighbors = self.board.get_neighbor_coords(x, y);
        for (nx, ny) in neighbors {
            if let Some(cell) = self.board.get_cell(nx, ny) {
                if cell.state == CellState::Hidden {
                    self.reveal_cell(nx, ny);
                }
            }
        }
    }
    pub fn set_flag(&mut self, x: usize, y: usize) {
        if self.state != GameState::Running {
            return;
        }

        if let Some(cell) = self.board.get_mut_cell(x, y) {
            if cell.state == CellState::Hidden {
                cell.state = CellState::Flagged;
                self.mines_left -= 1;
            } else if cell.state == CellState::Flagged {
                cell.state = CellState::Hidden;
                self.mines_left += 1;
            }
        }
    }
}
