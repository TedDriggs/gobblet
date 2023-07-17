use std::{
    error::Error,
    io::{self, Write},
};

use gobblin::Game;

mod ui;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();

    while game.outcome().is_none() {
        ui::draw_game(&game);

        print!("{:#}: ", game.next_player());
        io::stdout().flush()?;

        let mut next_move = format!("{:#} ", game.next_player());
        io::stdin().read_line(&mut next_move)?;

        let next_mv = match next_move.parse() {
            Ok(mv) => mv,
            Err(e) => {
                println!("Invalid move: {}", e);
                continue;
            }
        };

        if let Err(err) = game.submit(next_mv) {
            println!("Illegal move: {}", err);
        }
    }

    if let Some(victory) = game.outcome() {
        println!("{}", victory);
    } else {
        println!("No winner");
    }

    Ok(())
}
