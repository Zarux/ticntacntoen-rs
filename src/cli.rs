use clap::Parser;

use crate::{board::Player, filestate::FileState};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// The next player to make a move
    #[arg(short, long, value_enum, value_name = "PLAYER", default_value_t = Player::X)]
    pub player: Player,

    /// The board will be n x n size
    #[arg(short, value_name = "SIZE", default_value_t = 3)]
    pub n: i16,

    /// Will output only the next move, otherwise plays a full game
    #[arg(long, value_name = "NEXT_MOVE", default_value_t = false)]
    pub next_move: bool,

    /// The state file to load, will override n
    #[arg(long, value_name = "STATE_FILE")]
    pub state_file: Option<String>,

    /// How to store state
    #[arg(long, value_enum, value_name = "STATE_OUTPUT_TYPE", default_value_t = FileState::Board)]
    pub output_type: FileState,

    /// The state file to save
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_file: Option<String>,

    /// How many in a row to win
    #[arg(short, value_name = "WIN_CONDITION", default_value_t = 3)]
    pub k: i8,

    /// How many secodns the bot is allowed to think
    #[arg(short, long, value_name = "THINKING_SECONDS", default_value_t = 5)]
    pub think_time: u64,

    /// Will not print the board for each move
    #[arg(short, long, value_name = "SILENT", default_value_t = false)]
    pub silent: bool,
}
