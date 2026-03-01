mod board;
mod cli;
mod filestate;
mod mct_bot;

use clap::Parser;

use crate::board::{Board, Player};
use crate::mct_bot::Bot;

use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Args::parse();

    let mut b: Board;
    if let Some(f) = cli.state_file {
        let (n, cells) = filestate::get_cells(f.as_str())?;
        b = Board::new_from_state(n as i16, cli.k, cells);
    } else {
        b = Board::new(cli.n, cli.k);
    }

    let mut bot = Bot::new(Duration::new(cli.think_time, 0));

    if !cli.next_move {
        return play(b, bot, cli.player, !cli.silent);
    }

    let next_move = bot.find_next_move(&b, cli.player)?;
    let winner = b.apply_move(next_move, cli.player)?;

    if cli.output_file.is_none() {
        print!("next_move={next_move}");
        if winner.is_some() {
            let winner = winner.unwrap();
            print!(" winner={winner}");
        }

        print!("\n");
        return Ok(());
    }

    Ok(())
}

fn play(
    board: Board,
    bot: Bot,
    starting_player: Player,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let mut bot = bot;
    let mut board = board;
    let mut winner: Option<Player>;
    let mut player = starting_player;

    loop {
        if board.is_tie() {
            println!("no dinner");
            return Ok(());
        }

        let nm = bot.find_next_move(&board, player)?;
        if verbose {
            print!("Found move {nm}\n");
        }

        winner = board.apply_move(nm, player)?;
        if verbose {
            board.print();
        }

        player = player.next();

        if let Some(winner) = winner {
            println!("chicken dinner {winner}");
            return Ok(());
        }
    }
}
