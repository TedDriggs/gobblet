use ansi_escapes::{CursorTo, EraseScreen};
use gobblin::Game;

pub fn draw_game(game: &Game) {
    println!("{}{}{}", EraseScreen, CursorTo::TopLeft, game.board())
}
