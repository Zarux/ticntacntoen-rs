#[derive(Clone, Copy, PartialEq, strum_macros::Display)]
pub enum Player {
    #[strum(serialize = "\x1b[31mX\x1b[0m")]
    X,
    #[strum(serialize = "\x1b[32mO\x1b[0m")]
    O,
}

impl Player {
    pub fn next(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
