mod display;
mod game;
mod input;

use crate::display::Display;
use crate::game::Game;
use game::Status;
use input::Input;

fn main() {
    let mut game = Game::new();
    let mut display = display::tui::Display::new(game.state());
    let mut input = input::tui::Input::new();
    // let mut display = display::term::Display::new(game.state());
    // let mut input = input::term::Input::new();

    display.draw();

    loop {
        let input_res = input.get();

        display.on_input(&input_res);
        let (i, j) = match input_res {
            Ok(input::Result::Position(i, j)) => (i, j),
            Ok(input::Result::Exit) => return,
            _ => continue,
        };
        let status = game.apply(i, j);
        display.on_move(status);

        match status {
            Ok(game::Status::Ended(_)) => break,
            _ => {}
        };

        display.update(game.state());
        display.draw();
    }

    display.update(game.state());
    display.draw();

    input.wait_exit();
}
