use crate::board::Board;
use crate::cell::{CellContent, CellState, TOTAL_CELLS, WIDTH};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Difficulty {
    Easy,
    Hard,
}

impl Difficulty {
    pub fn mine_count(&self) -> usize {
        match self {
            Difficulty::Easy => 20,
            Difficulty::Hard => 50,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameStatus {
    MainMenu,
    Playing,
    Won,
    Lost,
}

pub struct Game {
    pub status: GameStatus,
    pub difficulty: Difficulty,
    pub board: Board,
    pub mines_left: usize,
    pub total_mines: usize,
    pub mines_placed: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            status: GameStatus::MainMenu,
            difficulty: Difficulty::Easy,
            board: Board::new(),
            mines_left: 0,
            total_mines: 0,
            mines_placed: false,
        }
    }

    pub fn start_game(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.total_mines = difficulty.mine_count();
        self.mines_left = self.total_mines;
        self.board = Board::new();
        self.mines_placed = false;
        self.status = GameStatus::Playing;
    }

    pub fn place_mines(&mut self, safe_x: usize, safe_y: usize) {
        let mut rng = thread_rng();
        let mut indices: Vec<usize> = (0..TOTAL_CELLS).collect();

        if let Some(safe_idx) = (safe_y * WIDTH + safe_x).into() {
            if safe_idx < indices.len() {
                indices.remove(safe_idx);
            }
        }

        indices.shuffle(&mut rng);

        for &idx in indices.iter().take(self.total_mines) {
            self.board.grid[idx].content = CellContent::Mine;
        }
        self.mines_placed = true;
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) -> bool {
        if self.status != GameStatus::Playing {
            return true;
        }

        if !self.mines_placed {
            self.place_mines(x, y);
        }

        let cell_state = match self.board.get_cell(x, y) {
            Some(c) => c.state,
            None => return true,
        };

        if cell_state == CellState::Revealed || cell_state == CellState::Flagged {
            return true;
        }

        let is_mine = match self.board.get_cell(x, y) {
            Some(c) => c.content == CellContent::Mine,
            None => return true,
        };

        if is_mine {
            if let Some(cell) = self.board.get_mut_cell(x, y) {
                cell.state = CellState::Revealed;
            }
            self.status = GameStatus::Lost;
            return false;
        }

        let mine_count = self.board.count_neighbor_mines(x, y);

        if let Some(cell) = self.board.get_mut_cell(x, y) {
            cell.content = CellContent::Empty(mine_count);
            cell.state = CellState::Revealed;
        }

        if mine_count == 0 {
            self.reveal_neighbors(x, y);
        }

        if self.check_win_condition() {
            self.status = GameStatus::Won;
            return false;
        }

        true
    }

    fn reveal_neighbors(&mut self, x: usize, y: usize) {
        let neighbors = self.board.get_neighbor_coords(x, y);
        for (nx, ny) in neighbors {
            if let Some(cell) = self.board.get_cell(nx, ny) {
                if cell.state == CellState::Hidden {
                    self.reveal_cell(nx, ny);
                }
            }
        }
    }

    pub fn check_win_condition(&self) -> bool {
        self.board
            .grid
            .iter()
            .all(|cell| cell.content == CellContent::Mine || cell.state == CellState::Revealed)
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.status != GameStatus::Playing {
            return;
        }

        if let Some(cell) = self.board.get_mut_cell(x, y) {
            match cell.state {
                CellState::Hidden => {
                    cell.state = CellState::Flagged;
                    self.mines_left = self.mines_left.saturating_sub(1);
                }
                CellState::Flagged => {
                    cell.state = CellState::Hidden;
                    self.mines_left = self.mines_left.saturating_add(1);
                }
                CellState::Revealed => {}
            }
        }
    }
}
