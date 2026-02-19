mod board;
mod mct_bot;

use crate::board::{Board, Player};
use crate::mct_bot::Bot;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut b = Board::new(5, 4);
    let mut winner: Option<Player>;

    let mut bot_player = Bot::new();

    b.print();

    let mut p = Player::X;

    loop {
        if b.is_tie() {
            println!("no dinner");
            return Ok(());
        }

        let nm = bot_player.find_next_move(&b, p)?;
        print!("found move {nm}\n");
        winner = b.apply_move(nm, p)?;
        b.print();
        p = p.next();

        if winner.is_some() {
            let winner = winner.unwrap();
            println!("chicken dinner {winner}");
            return Ok(());
        }
    }
}
