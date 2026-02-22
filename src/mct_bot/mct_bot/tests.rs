use std::time::Duration;

use crate::{
    board::{Board, Player},
    mct_bot,
};

use crate::mct_bot::mct_bot::BotBoard;
use crate::mct_bot::mct_bot::Node;

const TEST_THINKING_TIME: Duration = Duration::new(5, 0);

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
    fn find_best_move_win() {
        let board = "
        [ ][ ][ ][ ][ ]
        [ ][X][O][ ][ ]
        [ ][ ][X][O][ ]
        [ ][ ][ ][X][O]
        [ ][ ][ ][ ][ ]
        ";
        let mut test_board = Board::new(5, 4);
        for (m, p) in str_board_to_moves(board) {
            test_board.apply_move(m, p).expect("move should be valid");
        }

        let mut b = mct_bot::Bot::new(TEST_THINKING_TIME);
        let m = b
            .find_next_move(&test_board, crate::board::Player::X)
            .expect("could find move");

        assert_eq!(m, 0);
    }

    #[test]
    fn find_best_move_block() {
        let board = "
        [ ][ ][ ][ ][ ]
        [ ][X][O][ ][ ]
        [ ][ ][X][O][ ]
        [ ][X][ ][ ][O]
        [ ][ ][ ][ ][ ]
        ";
        let mut test_board = Board::new(5, 4);
        for (m, p) in str_board_to_moves(board) {
            test_board.apply_move(m, p).expect("move should be valid");
        }

        let mut b = mct_bot::Bot::new(TEST_THINKING_TIME);
        let m = b
            .find_next_move(&test_board, crate::board::Player::X)
            .expect("could find move");

        assert_eq!(m, 1);
    }

    #[test]
    fn find_best_move_block_open_three() {
        let board = "
        [ ][ ][ ][ ][ ][ ][ ][ ][ ]
        [ ][ ][ ][ ][ ][ ][ ][ ][ ]
        [ ][ ][ ][ ][ ][X][ ][ ][ ]
        [ ][ ][ ][O][ ][X][ ][ ][ ]
        [ ][ ][ ][ ][O][ ][ ][ ][ ]
        [ ][ ][ ][X][ ][O][ ][ ][ ]
        [ ][ ][ ][ ][ ][ ][ ][ ][ ]
        [ ][ ][ ][ ][ ][ ][ ][ ][ ]
        [ ][ ][ ][ ][ ][ ][ ][ ][ ]
        ";
        let mut test_board = Board::new(9, 5);
        for (m, p) in str_board_to_moves(board) {
            test_board.apply_move(m, p).expect("move should be valid");
        }

        let mut b = mct_bot::Bot::new(TEST_THINKING_TIME);
        let m = b
            .find_next_move(&test_board, crate::board::Player::X)
            .expect("could find move");

        println!("FOUND {m}");
        assert!(m == 20 || m == 60);
    }

    #[test]
    fn expansion_tactical() {
        let board = "
        [ ][ ][ ][ ][ ]
        [ ][X][O][ ][ ]
        [ ][ ][X][O][ ]
        [ ][X][ ][ ][O]
        [ ][ ][ ][ ][ ]
        ";
        let mut raw_test_board = Board::new(5, 4);
        let mut test_board = BotBoard::new(raw_test_board.clone());
        for (m, p) in str_board_to_moves(board) {
            raw_test_board
                .apply_move(m, p)
                .expect("move should be valid");
        }
        test_board.update_board(raw_test_board.clone());

        let mut b = mct_bot::Bot::new(TEST_THINKING_TIME);
        let mut root = Node::new(Player::X);

        root.untried_moves = test_board.legal_moves();
        b.nodes.push(root);

        let (n_i, _) = b.expand(0, &mut test_board, Player::X);

        assert!(b.nodes[n_i].game_move == Some(1))
    }
}
