use crate::board::Board;

pub struct BotBoard {
    pub board: Board,
}

impl BotBoard {
    pub fn new(board: Board) -> Self {
        Self { board: board }
    }
}

pub fn legal_moves(board: &Board) -> Vec<usize> {
    let mut untried_moves = Vec::with_capacity(board.cells().len());
    untried_moves.extend(board.cells().iter().enumerate().filter_map(|(m, cell)| {
        if cell.is_none() {
            return Some(m);
        }
        None
    }));
    return untried_moves;
}
