use std::error::Error;
use std::time::{Duration, SystemTime};

use rand::seq::IndexedRandom;
use rand::{random_range, rng};

use crate::board::{Board, Player};
use crate::mct_bot::bot_board::BotBoard;

#[cfg(test)]
mod tests;

#[derive(strum_macros::Display, Debug)]
pub enum BotError {
    NoMoreMoves,
}

impl Error for BotError {}

const EXPLORATION_PARAM: f32 = 1.414;
const NEIGHBOUR_CHANCE: f32 = 0.6;
const WIN_VALUE: f32 = 1.;
const DRAW_VALUE: f32 = 0.6;

#[derive(Clone)]
pub struct Node {
    parent: Option<usize>,
    children: Vec<usize>,

    game_move: Option<i16>,
    wins: f32,
    visits: i32,
    untried_moves: Vec<i16>,
    player: Player,
}

impl Node {
    pub fn new(p: Player) -> Self {
        Self {
            parent: None,
            children: vec![],
            wins: 0.0,
            visits: 0,
            untried_moves: vec![],
            game_move: None,
            player: p,
        }
    }

    pub fn can_expand(&self) -> bool {
        let max_children = 2.0 * (self.visits as f32).sqrt();
        self.untried_moves.len() > 0 && (self.children.len() as f32) < max_children
    }
}

pub struct Bot {
    nodes: Vec<Node>,
    thinking_time: Duration,
    turn: usize,
}

impl Bot {
    pub fn new(thinking_time: Duration) -> Self {
        Self {
            nodes: vec![],
            thinking_time: thinking_time,
            turn: 0,
        }
    }

    fn uct_value(&self, node_index: usize, p_v_ln: f32) -> f32 {
        unsafe {
            let current = self.nodes.get_unchecked(node_index);
            if current.visits == 0 {
                return f32::INFINITY;
            }

            let visits = current.visits as f32;
            let wins = current.wins;

            wins / visits + EXPLORATION_PARAM * (p_v_ln / visits).sqrt()
        }
    }

    fn select_child(&self, node_index: usize) -> usize {
        unsafe {
            let current = self.nodes.get_unchecked(node_index);
            let current_visits_ln = (current.visits as f32).ln();

            current
                .children
                .iter()
                .max_by(|&&a, &&b| {
                    let u_a = self.uct_value(a, current_visits_ln);
                    let u_b = self.uct_value(b, current_visits_ln);

                    u_a.partial_cmp(&u_b).unwrap_or(std::cmp::Ordering::Equal)
                })
                .copied()
                .expect("node without children")
        }
    }

    fn expand(
        &mut self,
        node_index: usize,
        board: &mut BotBoard,
        player: Player,
    ) -> (usize, Option<Player>) {
        let mut idx = Some(random_range(0..self.nodes[node_index].untried_moves.len()));
        let mut tactical_moves: Vec<i16> = vec![];
        let mut neighbour_moves: Vec<i16> = vec![];

        self.nodes[node_index].untried_moves.iter().for_each(|&m| {
            if board.is_tactical_move(m, player) {
                tactical_moves.push(m);
                return;
            }

            if tactical_moves.is_empty() && board.has_neighbour(m) {
                neighbour_moves.push(m);
            }
        });

        if !tactical_moves.is_empty() {
            let m = tactical_moves[random_range(0..tactical_moves.len())];
            idx = Some(
                self.nodes[node_index]
                    .untried_moves
                    .iter()
                    .position(|&um| um == m)
                    .expect("untried moves should have the tactical move"),
            );
        }

        if idx.is_none()
            && rand::random_bool(NEIGHBOUR_CHANCE as f64)
            && !neighbour_moves.is_empty()
        {
            let m = neighbour_moves[random_range(0..neighbour_moves.len())];
            idx = Some(
                self.nodes[node_index]
                    .untried_moves
                    .iter()
                    .position(|&um| um == m)
                    .expect("untried moves should have the tactical move"),
            );
        }

        let m = self.nodes[node_index]
            .untried_moves
            .swap_remove(idx.unwrap());

        let winner = board
            .board
            .apply_move(m, player)
            .expect("legal move should be valid during expansion");

        let child_index = self.nodes.len();
        let child = Node {
            parent: Some(node_index),
            game_move: Some(m.try_into().expect("game move should fit in i16")),
            children: vec![],
            wins: 0.,
            visits: 0,
            untried_moves: board.legal_moves(),
            player: player,
        };

        self.nodes.push(child);
        self.nodes[node_index].children.push(child_index);

        (child_index, winner)
    }

    fn rollout(&self, board: &mut BotBoard, mut player: Player) -> Option<Player> {
        let mut moves = board.legal_moves();

        loop {
            if moves.is_empty() {
                return None;
            }

            let idx = random_range(0..moves.len());
            let winner = board
                .board
                .apply_move(moves[idx], player)
                .expect("legal move");

            if winner.is_some() {
                return winner;
            }

            moves.swap_remove(idx);

            player = player.next();
        }
    }

    fn backpropagate(&mut self, mut node_index: usize, winner: Option<Player>) {
        loop {
            self.nodes[node_index].visits += 1;
            let val = match winner {
                Some(p) => {
                    if p == self.nodes[node_index].player {
                        WIN_VALUE
                    } else {
                        0.0
                    }
                }
                None => DRAW_VALUE,
            };

            self.nodes[node_index].wins += val;

            if self.nodes[node_index].parent.is_none() {
                return;
            }

            node_index = self.nodes[node_index].parent.expect("node has parent");
        }
    }

    pub fn find_next_move(
        &mut self,
        original_board: &Board,
        player: Player,
    ) -> Result<i16, Box<dyn Error>> {
        self.nodes.clear();

        let mut board = BotBoard::new(original_board.clone());
        let legal_moves = board.legal_moves();
        self.turn = original_board.cells().len() - legal_moves.len();

        let (winning_move, blocking_moves) = board.terminating_moves(player);

        if winning_move.is_some() {
            return Ok(winning_move.unwrap());
        }

        if blocking_moves.len() == 1 {
            return Ok(blocking_moves[0]);
        }

        let mut root = Node::new(player);
        root.untried_moves = legal_moves;
        if root.untried_moves.is_empty() {
            return Err(Box::from(BotError::NoMoreMoves));
        }

        root.game_move = Some(
            root.untried_moves[0]
                .try_into()
                .expect("game move should fit in i16"),
        );
        self.nodes.push(root);

        let mut iterations = 0;
        let now = SystemTime::now();
        'iter_loop: loop {
            if now.elapsed().expect("time working") > self.thinking_time {
                break 'iter_loop;
            };
            iterations += 1;

            board.update_board(original_board.clone());

            let mut current_player = player;
            let mut current_node_index = 0;

            //SELECTION
            while !self.nodes[current_node_index].can_expand()
                && !self.nodes[current_node_index].children.is_empty()
            {
                current_node_index = self.select_child(current_node_index);

                let game_move = self.nodes[current_node_index]
                    .game_move
                    .expect("node with move");

                let winner = board
                    .board
                    .apply_move(
                        game_move.try_into().expect("game move should fit in i16"),
                        current_player,
                    )
                    .expect("valid move");

                if winner.is_some() {
                    self.backpropagate(current_node_index, winner);
                    continue 'iter_loop;
                }

                current_player = current_player.next();
            }

            //EXPANSION
            if self.nodes[current_node_index].can_expand() {
                let (new_node_index, winner) =
                    self.expand(current_node_index, &mut board, current_player);

                if winner.is_some() {
                    self.backpropagate(current_node_index, winner);
                    continue 'iter_loop;
                }

                current_node_index = new_node_index;
                current_player = current_player.next();
            }

            //SIMULATION
            let winner = self.rollout(&mut board, current_player);

            //BACKPROPAGATION
            self.backpropagate(current_node_index, winner);
        }

        let &best_node = self.nodes[0]
            .children
            .iter()
            .max_by(|&&a, &&b| self.nodes[a].visits.cmp(&self.nodes[b].visits))
            .expect("nodes should not be empty");

        let mut best_move = self.nodes[best_node]
            .game_move
            .expect("node should have move");

        if !blocking_moves.is_empty() && !blocking_moves.contains(&best_move) {
            best_move = *blocking_moves
                .choose(&mut rng())
                .expect("blocking moves should not be empty");
        }

        println!("iterations: {iterations}");
        Ok(best_move)
    }
}
