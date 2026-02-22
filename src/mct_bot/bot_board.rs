use crate::board::{Board, Player};

#[cfg(test)]
mod tests;

pub struct BotBoard {
    pub board: Board,
    x_list: Vec<i16>,
    y_list: Vec<i16>,
}

impl BotBoard {
    pub fn new(board: Board) -> Self {
        let x_list: Vec<i16> = (0..board.n() * board.n()).map(|i| i % board.n()).collect();
        let y_list: Vec<i16> = (0..board.n() * board.n()).map(|i| i / board.n()).collect();
        Self {
            board: board,
            x_list: x_list,
            y_list: y_list,
        }
    }

    pub fn update_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn legal_moves(&self) -> Vec<i16> {
        let mut untried_moves = Vec::with_capacity(self.board.cells().len());
        untried_moves.extend(
            self.board
                .cells()
                .iter()
                .enumerate()
                .filter_map(|(m, cell)| {
                    if cell.is_none() {
                        return Some(m as i16);
                    }
                    None
                }),
        );
        return untried_moves;
    }

    pub fn is_tactical_move(&mut self, m: i16, player: Player) -> bool {
        let next_winner = self
            .board
            .apply_move(m, player.next())
            .expect("move should be valid");
        self.board
            .undo_last_move()
            .expect("undo move should be valid");

        if next_winner.is_some() {
            return true;
        }

        let self_winner = self
            .board
            .apply_move(m, player)
            .expect("move should be valid");
        self.board
            .undo_last_move()
            .expect("undo move should be valid");

        self_winner.is_some()
    }

    pub fn has_neighbour(&mut self, m: i16) -> bool {
        let cells = self.board.cells();
        let n = self.board.n();
        let x: i16;
        let y: i16;
        unsafe {
            x = *self.x_list.get_unchecked(m as usize);
            y = *self.y_list.get_unchecked(m as usize);
        }

        for dy in -1..=1 {
            let ny = y + dy;
            if ny < 0 || ny >= n {
                continue;
            }

            for dx in -1..=1 {
                let nx = x + dx;
                if (nx < 0 || nx >= n) || dx == 0 && dy == 0 {
                    continue;
                }

                unsafe {
                    if cells.get_unchecked((ny * n + nx) as usize).is_some() {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn terminating_moves(&mut self, player: Player) -> (Option<i16>, Vec<i16>) {
        let mut blocking_moves: Vec<i16> = vec![];

        for (m, cell) in self.board.clone().cells().iter().enumerate() {
            if cell.is_some() {
                continue;
            }

            let m = m as i16;

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
