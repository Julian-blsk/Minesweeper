mod board;
mod cell;
mod game;
mod game_ui;

slint::include_modules!();

use std::{cell::RefCell, rc::Rc};

use game::Game;
use game_ui::GameUi;
use slint::ModelRc;

fn main() {
    let ui = MainWindow::new().unwrap();
    let game_ui = Rc::new(RefCell::new(GameUi::new()));
    ui.set_cells(ModelRc::from(game_ui.borrow().ui_cells.clone()));

    let ui_handle = ui.as_weak();
    let game_handle = game_ui.clone();

    ui.on_cell_clicked(move |x, y| {
        let ui = ui_handle.upgrade().unwrap();
        let mut game = game_handle.borrow_mut();
        game.handle_click(x as usize, y as usize, &ui);
    });

    let ui_right_handle = ui.as_weak();
    let right_game_handle = game_ui.clone();

    ui.on_cell_right_clicked(move |x, y| {
        let ui = ui_right_handle.upgrade().unwrap();
        let mut game = right_game_handle.borrow_mut();
        game.handle_right_click(x as usize, y as usize, &ui);
    });

    game_ui.borrow().update_ui(&ui);
    ui.run().unwrap();
}
