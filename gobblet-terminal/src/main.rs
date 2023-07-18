use std::{
    error::Error,
    io::{self, Write},
};

use gobblet::Game;

use crate::ui::reset_screen;

mod ui;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();

    let mut last_move_error: Option<Box<dyn std::fmt::Display>> = None;

    while game.outcome().is_none() {
        reset_screen();

        println!("{}", game.board());

        if let Some(err) = last_move_error.take() {
            println!("Last move invalid: {}. Please try again.", err);
        }

        print!("{:#}: ", game.next_player());
        io::stdout().flush()?;

        let mut next_move = format!("{:#} ", game.next_player());
        io::stdin().read_line(&mut next_move)?;

        let next_mv = match next_move.parse() {
            Ok(mv) => mv,
            Err(e) => {
                last_move_error = Some(Box::new(e));
                continue;
            }
        };

        if let Err(err) = game.submit(next_mv) {
            last_move_error = Some(Box::new(err));
        }
    }

    if let Some(victory) = game.outcome() {
        println!("{}", victory);
    } else {
        println!("No winner");
    }

    Ok(())
}
