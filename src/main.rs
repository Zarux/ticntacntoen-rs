mod board;
mod mct_bot;

use crate::board::{Board, Player};
use crate::mct_bot::Bot;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut b = Board::new(3, 3);
    let mut p = Player::X;
    let mut winner: Option<Player>;

    let mut bot_player = Bot::new();

    b.print();

    for m in [0, 2, 3, 5, 6] {
        let nm = bot_player.find_next_move(&b, Player::O);
        winner = b.apply_move(m, p)?;
        b.print();
        p = p.next();
        if winner.is_none() {
            continue;
        }

        let winner = winner.unwrap();
        println!("chicken dinner {winner}");

        return Ok(());
    }

    Ok(())
}
