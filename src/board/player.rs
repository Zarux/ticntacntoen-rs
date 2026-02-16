#[derive(Clone, Copy, PartialEq, strum_macros::Display)]
pub enum Player {
    X,
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
