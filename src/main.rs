use std::error::Error;

use gobblin::Game;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();

    while game.outcome().is_none() {
        println!("{}", game.board());

        let mut next_move = String::new();
        std::io::stdin().read_line(&mut next_move)?;
        game.submit(next_move.parse()?)?;
    }

    if let Some(victory) = game.outcome() {
        println!("{}", victory);
    } else {
        println!("No winner");
    }

    Ok(())
}
