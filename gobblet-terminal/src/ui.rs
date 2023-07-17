use ansi_escapes::{CursorTo, EraseScreen};
use gobblet::Game;

pub fn draw_game(game: &Game) {
    println!("{}{}{}", EraseScreen, CursorTo::TopLeft, game.board())
}
