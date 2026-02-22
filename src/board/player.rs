#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum, strum_macros::Display)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn pretty(self) -> String {
        match self {
            Player::X => String::from("\x1b[31mX\x1b[0m"),
            Player::O => String::from("\x1b[32mO\x1b[0m"),
        }
    }

    pub fn next(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
