use std::error::Error;
use std::time::{Duration, SystemTime};

use rand::random_range;
use rand::seq::IndexedRandom;

use crate::board::{Board, Player};
use crate::mct_bot::bot_board::{BotBoard, legal_moves};

#[derive(strum_macros::Display, Debug)]
pub enum BotError {
    NoMoreMoves,
}

impl Error for BotError {}

const EXPLORATION_PARAM: f64 = 1.414;
const MAX_THINKING_TIME: Duration = Duration::new(10, 0);
const WIN_VALUE: f64 = 1.;
const DRAW_VALUE: f64 = 0.6;

#[derive(Clone)]
pub struct Node {
    parent: Option<usize>,
    children: Vec<usize>,

    game_move: Option<usize>,
    wins: f64,
    visits: i32,
    untried_moves: Vec<usize>,
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
        let max_children = 2.0 * (self.visits as f64).sqrt();
        self.untried_moves.len() > 0 && (self.children.len() as f64) < max_children
    }
}

pub struct Bot {
    nodes: Vec<Node>,
    thinking_time: Duration,
}

impl Bot {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            thinking_time: MAX_THINKING_TIME,
        }
    }

    fn uct_value(&self, node_index: usize) -> f64 {
        let node = &self.nodes[node_index];
        if node.visits == 0 {
            return f64::INFINITY;
        }

        let parent_visits = match node.parent {
            Some(idx) => self.nodes[idx].visits as f64,
            None => 0.0,
        };

        (node.wins / node.visits as f64)
            + EXPLORATION_PARAM * (parent_visits.ln() / node.visits as f64).sqrt()
    }

    fn select_child(&self, node_index: usize) -> usize {
        let current = &self.nodes[node_index];

        current
            .children
            .iter()
            .max_by(|&&a, &&b| {
                let u_a = self.uct_value(a);
                let u_b = self.uct_value(b);

                u_a.partial_cmp(&u_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
            .expect("node without children")
    }

    fn expand(
        &mut self,
        node_index: usize,
        board: &mut BotBoard,
        player: Player,
    ) -> (usize, Option<Player>) {
        let idx = random_range(0..self.nodes[node_index].untried_moves.len());
        let m = self.nodes[node_index].untried_moves.swap_remove(idx);

        let winner = board
            .board
            .apply_move(m, player)
            .expect("legal move during expansion");

        let child_index = self.nodes.len();
        let child = Node {
            parent: Some(node_index),
            game_move: Some(m),
            children: vec![],
            wins: 0.,
            visits: 0,
            untried_moves: legal_moves(&board.board),
            player: player,
        };

        self.nodes.push(child);
        self.nodes[node_index].children.push(child_index);

        (child_index, winner)
    }

    fn rollout(&self, board: &mut BotBoard, mut player: Player) -> Option<Player> {
        loop {
            let moves = legal_moves(&board.board);
            if moves.is_empty() {
                return None;
            }

            let mut rng = rand::rng();
            let &m = moves.choose(&mut rng).expect("moves not empty");
            let winner = board.board.apply_move(m, player).expect("legal move");
            if winner.is_some() {
                return winner;
            }

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
        board: &Board,
        player: Player,
    ) -> Result<usize, Box<dyn Error>> {
        self.nodes.clear();

        let mut root = Node::new(player);
        root.untried_moves = legal_moves(&board);
        if root.untried_moves.is_empty() {
            return Err(Box::from(BotError::NoMoreMoves));
        }

        root.game_move = Some(root.untried_moves[0]);
        self.nodes.push(root);

        let now = SystemTime::now();
        'iter_loop: loop {
            if now.elapsed().expect("time working") > self.thinking_time {
                break 'iter_loop;
            };

            let mut board = BotBoard::new(board.clone());

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
                    .apply_move(game_move, current_player)
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

            //BACKPROPOGATION
            self.backpropagate(current_node_index, winner);
        }

        let &best_node = self.nodes[0]
            .children
            .iter()
            .max_by(|&&a, &&b| self.nodes[a].visits.cmp(&self.nodes[b].visits))
            .expect("nodes should not be empty");

        Ok(self.nodes[best_node]
            .game_move
            .expect("node should have move"))
    }
}
