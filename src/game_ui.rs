use crate::cell::{CellContent, CellState, HEIGHT, WIDTH};
use crate::game::{Difficulty, Game, GameStatus};
use slint::{Model, SharedString, VecModel};
use std::rc::Rc;

use crate::MainWindow;

use crate::CellData;

pub struct GameUi {
    pub game: Game,
    pub ui_cells: Rc<VecModel<CellData>>,
}

impl GameUi {
    pub fn new() -> Self {
        let game = Game::new();
        let cells = (0..(WIDTH * HEIGHT))
            .map(|_| CellData {
                x: 0,
                y: 0,
                is_mine: false,
                is_flagged: false,
                is_revealed: false,
                number: SharedString::from(""),
            })
            .collect::<Vec<_>>();

        Self {
            game,
            ui_cells: Rc::new(VecModel::from(cells)),
        }
    }

    pub fn start_game(&mut self, difficulty: Difficulty, ui: &MainWindow) {
        self.game.start_game(difficulty);
        self.sync_board_to_ui();
        self.update_ui_status(ui);
    }

    pub fn sync_board_to_ui(&self) {
        for (i, cell) in self.game.board.grid.iter().enumerate() {
            let x = (i % WIDTH) as i32;
            let y = (i / WIDTH) as i32;

            let (is_mine, number) = match cell.content {
                CellContent::Empty(n) => (false, SharedString::from(n.to_string())),
                CellContent::Mine => (true, SharedString::from("")),
            };

            let data = CellData {
                x,
                y,
                is_mine,
                is_flagged: cell.state == CellState::Flagged,
                is_revealed: cell.state == CellState::Revealed,
                number,
            };
            self.ui_cells.set_row_data(i, data);
        }
    }

    pub fn update_ui_status(&self, ui: &MainWindow) {
        let status_text = match self.game.status {
            GameStatus::MainMenu => "Hauptmenü",
            GameStatus::Lost => "Verloren",
            GameStatus::Won => "Gewonnen",
            GameStatus::Playing => "Laufend",
        };

        ui.set_game_status(SharedString::from(status_text));
        ui.set_mines_left(self.game.mines_left as i32);
    }

    pub fn handle_click(&mut self, x: usize, y: usize, ui: &MainWindow) {
        self.game.reveal_cell(x, y);
        self.sync_board_to_ui();
        self.update_ui_status(ui);
    }

    pub fn handle_right_click(&mut self, x: usize, y: usize, ui: &MainWindow) {
        self.game.toggle_flag(x, y);
        self.sync_board_to_ui();
        self.update_ui_status(ui);
    }
}
