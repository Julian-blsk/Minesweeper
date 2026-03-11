use std::rc::Rc;

use slint::{Model, SharedString, VecModel};

use crate::{
    CellData, Game, MainWindow,
    cell::{CellContent, CellState, WIDTH},
    game::GameState,
};

pub struct GameUi {
    pub game: Game,
    pub ui_cells: Rc<VecModel<CellData>>,
}

impl GameUi {
    pub fn new() -> Self {
        let game = Game::new();

        let mut cells = Vec::new();
        for i in 0..(WIDTH * WIDTH) {
            cells.push(CellData {
                x: (i % WIDTH) as i32,
                y: (i / WIDTH) as i32,
                is_mine: false,
                is_flagged: false,
                is_revealed: false,
                number: SharedString::from(""),
            });
        }

        let ui_cells = Rc::new(VecModel::from(cells));

        Self { game, ui_cells }
    }

    pub fn synch_board_to_ui(&self) {
        for i in 0..(WIDTH * WIDTH) {
            let cell = self.game.board.grid[i];
            let x = (i % WIDTH) as i32;
            let y = (i / WIDTH) as i32;

            let (is_mine, number) = match cell.content {
                CellContent::Empty(n) => (false, SharedString::from(n.to_string())),
                CellContent::Mine => (true, SharedString::from("")),
            };

            let is_revealed = cell.state == CellState::Revealed;
            let is_flagged = cell.state == CellState::Flagged;

            let data = CellData {
                x,
                y,
                is_mine,
                is_flagged,
                is_revealed,
                number,
            };
            self.ui_cells.set_row_data(i, data);
        }
    }

    pub fn update_ui(&self, ui: &MainWindow) {
        let status = match self.game.state {
            GameState::Lost => "Verloren",
            GameState::Running => "Laufend",
            GameState::Won => "Gewonnen",
        };
        ui.set_game_status(SharedString::from(status));
        ui.set_mines_left(self.game.mines_left as i32);
    }

    pub fn handle_click(&mut self, x: usize, y: usize, ui: &MainWindow) {
        if self.game.state != GameState::Running {
            return;
        }

        self.game.reveal_cell(x, y);
        self.synch_board_to_ui();
        self.update_ui(ui);
    }

    pub fn handle_right_click(&mut self, x: usize, y: usize, ui: &MainWindow) {
        if self.game.state != GameState::Running {
            return;
        }

        self.game.set_flag(x, y);
        self.synch_board_to_ui();
        self.update_ui(ui);
    }
}
