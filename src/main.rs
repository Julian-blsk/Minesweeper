mod board;
mod cell;
mod game;
mod game_ui;

slint::include_modules!();

use game::Difficulty;
use game_ui::GameUi;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let ui = MainWindow::new().expect("Failed to create UI");
    let game_ui = Rc::new(RefCell::new(GameUi::new()));

    ui.set_cells(slint::ModelRc::from(game_ui.borrow().ui_cells.clone()));
    game_ui.borrow().update_ui_status(&ui);

    {
        let ui_handle = ui.as_weak();
        let game_handle = game_ui.clone();
        ui.on_start_easy(move || {
            if let Some(ui_instance) = ui_handle.upgrade() {
                let mut game = game_handle.borrow_mut();
                game.start_game(Difficulty::Easy, &ui_instance);
            }
        });
    }

    {
        let ui_handle = ui.as_weak();
        let game_handle = game_ui.clone();
        ui.on_start_hard(move || {
            if let Some(ui_instance) = ui_handle.upgrade() {
                let mut game = game_handle.borrow_mut();
                game.start_game(Difficulty::Hard, &ui_instance);
            }
        });
    }

    {
        let ui_handle = ui.as_weak();
        let game_handle = game_ui.clone();
        ui.on_cell_clicked(move |x, y| {
            if let Some(ui_instance) = ui_handle.upgrade() {
                let mut game = game_handle.borrow_mut();
                game.handle_click(x as usize, y as usize, &ui_instance);
            }
        });
    }

    {
        let ui_handle = ui.as_weak();
        let game_handle = game_ui.clone();
        ui.on_cell_right_clicked(move |x, y| {
            if let Some(ui_instance) = ui_handle.upgrade() {
                let mut game = game_handle.borrow_mut();
                game.handle_right_click(x as usize, y as usize, &ui_instance);
            }
        });
    }

    ui.run().expect("UI Loop failed");
}
