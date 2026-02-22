use crate::board::{Board, Player};
use crate::mct_bot::bot_board::BotBoard;

#[cfg(test)]
fn str_board_to_moves(board: &str) -> Vec<(i16, Player)> {
    let mut moves = vec![];
    let mut m = -1;
    let mut prev_c = ' ';
    for c in board.trim().chars() {
        if c == ' ' && prev_c == '[' {
            m += 1;
        } else if c == 'X' {
            m += 1;
            moves.push((m, Player::X));
        } else if c == 'O' {
            m += 1;
            moves.push((m, Player::O));
        }
        prev_c = c;
    }

    moves
}

mod tests {
    use super::*;

    #[test]
    fn legal_moves() {
        let board = "
        [ ][ ][ ]
        [ ][X][O]
        [ ][ ][X]
        ";
        let mut raw_test_board = Board::new(3, 3);
        let mut test_board = BotBoard::new(raw_test_board.clone());
        for (m, p) in str_board_to_moves(board) {
            raw_test_board
                .apply_move(m, p)
                .expect("move should be valid");
        }
        test_board.update_board(raw_test_board);

        let legal_moves = test_board.legal_moves();

        assert_eq!(legal_moves, [0, 1, 2, 3, 6, 7]);
    }

    #[test]
    fn is_tactical_move() {
        let board = "
        [ ][ ][ ]
        [ ][X][O]
        [ ][ ][X]
        ";
        let mut raw_test_board = Board::new(3, 3);
        let mut test_board = BotBoard::new(raw_test_board.clone());
        for (m, p) in str_board_to_moves(board) {
            raw_test_board
                .apply_move(m, p)
                .expect("move should be valid");
        }
        test_board.update_board(raw_test_board);

        let is_tactical = test_board.is_tactical_move(0, Player::X);
        assert_eq!(is_tactical, true);

        let is_tactical = test_board.is_tactical_move(1, Player::X);
        assert_eq!(is_tactical, false);

        let is_tactical = test_board.is_tactical_move(0, Player::O);
        assert_eq!(is_tactical, true);

        let is_tactical = test_board.is_tactical_move(1, Player::O);
        assert_eq!(is_tactical, false);
    }

    #[test]
    fn has_neighbour() {
        let board = "
        [ ][ ][ ]
        [ ][ ][ ]
        [O][ ][X]
        ";
        let mut raw_test_board = Board::new(3, 3);
        let mut test_board = BotBoard::new(raw_test_board.clone());
        for (m, p) in str_board_to_moves(board) {
            raw_test_board
                .apply_move(m, p)
                .expect("move should be valid");
        }
        test_board.update_board(raw_test_board);

        let has_neighour = test_board.has_neighbour(1);
        assert_eq!(has_neighour, false);

        let has_neighour = test_board.has_neighbour(4);
        assert_eq!(has_neighour, true);

        let has_neighour = test_board.has_neighbour(5);
        assert_eq!(has_neighour, true);
    }

    #[test]
    fn terminating_moves_win() {
        let board = "
        [ ][ ][X][X]
        [ ][O][O][ ]
        [ ][ ][ ][ ]
        [ ][ ][ ][ ]
        ";
        let mut raw_test_board = Board::new(4, 3);
        let mut test_board = BotBoard::new(raw_test_board.clone());
        for (m, p) in str_board_to_moves(board) {
            raw_test_board
                .apply_move(m, p)
                .expect("move should be valid");
        }
        test_board.update_board(raw_test_board);

        let (winning, blocking) = test_board.terminating_moves(Player::X);

        assert!(winning == Some(1));
        assert!(blocking.is_empty());
    }

    #[test]
    fn terminating_moves_blocks() {
        let board = "
        [ ][X][ ][ ]
        [ ][O][O][ ]
        [ ][ ][X][ ]
        [ ][ ][ ][ ]
        ";
        let mut raw_test_board = Board::new(4, 3);
        let mut test_board = BotBoard::new(raw_test_board.clone());
        for (m, p) in str_board_to_moves(board) {
            raw_test_board
                .apply_move(m, p)
                .expect("move should be valid");
        }
        test_board.update_board(raw_test_board);

        let (winning, blocking) = test_board.terminating_moves(Player::X);

        assert_eq!(winning, None);
        assert_eq!(blocking, [4, 7]);
    }
}
