use crate::board::Board;

pub struct BotBoard {
    pub board: Board,
}

impl BotBoard {
    pub fn new(board: Board) -> Self {
        Self { board: board }
    }
}
