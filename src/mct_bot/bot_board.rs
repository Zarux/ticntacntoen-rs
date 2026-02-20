use crate::board::{Board, Player};

pub struct BotBoard {
    pub board: Board,
}

impl BotBoard {
    pub fn new(board: Board) -> Self {
        Self { board: board }
    }

    pub fn update_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn legal_moves(&self) -> Vec<usize> {
        legal_moves(&self.board)
    }

    pub fn is_tactical_move(&mut self, m: usize) -> bool {
        let x_winner = self
            .board
            .apply_move(m, Player::X)
            .expect("move should be valid");
        self.board
            .undo_last_move()
            .expect("undo move should be valid");

        if x_winner.is_some() {
            return true;
        }

        let o_winner = self
            .board
            .apply_move(m, Player::O)
            .expect("move should be valid");
        self.board
            .undo_last_move()
            .expect("undo move should be valid");

        if o_winner.is_some() {
            return true;
        }

        return false;
    }

    pub fn terminating_moves(&mut self, player: Player) -> (Option<usize>, Vec<usize>) {
        let mut blocking_moves: Vec<usize> = vec![];

        for (m, cell) in self.board.clone().cells().iter().enumerate() {
            if cell.is_some() {
                continue;
            }

            let winner = self
                .board
                .apply_move(m, player)
                .expect("move should be valid");
            self.board.undo_last_move().expect("undo should be valid");

            if winner.is_some_and(|p| p == player) {
                return (Some(m), vec![]);
            }

            let next_player = player.next();
            let winner = self
                .board
                .apply_move(m, next_player)
                .expect("move should be valid");
            self.board.undo_last_move().expect("undo should be valid");

            if winner.is_some_and(|p| p == next_player) {
                blocking_moves.push(m);
            }
        }

        (None, blocking_moves)
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
