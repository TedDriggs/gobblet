use std::error::Error;

use gobblin::Game;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();

    while game.outcome().is_none() {
        println!("{}", game.board());

        let mut next_move = String::new();
        std::io::stdin().read_line(&mut next_move)?;

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
