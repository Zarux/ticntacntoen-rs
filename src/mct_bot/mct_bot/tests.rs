use std::time::Duration;

use crate::{
    board::{Board, Player},
    mct_bot,
};

use crate::mct_bot::mct_bot::BotBoard;
use crate::mct_bot::mct_bot::Node;

const TEST_THINKING_TIME: Duration = Duration::new(5, 0);

mod tests {
    use crate::board;

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
        let (n, cells) = board::from_board_string_to_state(board);
        let mut test_board = Board::new(n as i16, 3);
        for (m, p) in cells.iter().enumerate() {
            if p.is_none() {
                continue;
            }

            test_board
                .apply_move(m as i16, p.expect("player should not be None"))
                .expect("move should be valid");
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
        let (n, cells) = board::from_board_string_to_state(board);
        let mut test_board = Board::new(n as i16, 3);
        for (m, p) in cells.iter().enumerate() {
            if p.is_none() {
                continue;
            }

            test_board
                .apply_move(m as i16, p.expect("player should not be None"))
                .expect("move should be valid");
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
        let (n, cells) = board::from_board_string_to_state(board);
        let mut test_board = Board::new(n as i16, 3);
        for (m, p) in cells.iter().enumerate() {
            if p.is_none() {
                continue;
            }

            test_board
                .apply_move(m as i16, p.expect("player should not be None"))
                .expect("move should be valid");
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
        let (n, cells) = board::from_board_string_to_state(board);
        let mut raw_test_board = Board::new(n as i16, 3);
        let mut test_board = BotBoard::new(raw_test_board.clone());
        for (m, p) in cells.iter().enumerate() {
            if p.is_none() {
                continue;
            }

            raw_test_board
                .apply_move(m as i16, p.expect("player should not be None"))
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
