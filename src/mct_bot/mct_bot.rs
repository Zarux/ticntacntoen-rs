use crate::board::{Board, Player};
use crate::mct_bot::bot_board::BotBoard;

const EXPLORATION_PARAM: f64 = 1.414;
const MAX_ITERATIONS: i32 = 10000;

#[derive(Clone)]
pub struct Node {
    parent: Option<usize>,
    children: Vec<usize>,

    game_move: Option<usize>,
    wins: f64,
    visits: i32,
    untried_moves: Vec<usize>,
    last_move: Option<usize>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            parent: None,
            children: vec![],
            wins: 0.0,
            visits: 0,
            untried_moves: vec![],
            last_move: None,
            game_move: None,
        }
    }

    pub fn can_expand(&self) -> bool {
        let max_children = 2.0 * (self.visits as f64).sqrt();
        self.untried_moves.len() > 0 && (self.children.len() as f64) < max_children
    }
}

pub struct Bot {
    nodes: Vec<Node>,
}

impl Bot {
    pub fn new() -> Self {
        Self { nodes: vec![] }
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

    fn expand(&self, node_index: usize, board: &BotBoard, player: Player) -> usize {
        1
    }

    fn rollout(&self, board: &BotBoard, player: Player) -> Player {
        player
    }

    fn backpropogate(&self, node_index: usize, player: Player) {}

    pub fn find_next_move(&mut self, board: &Board, player: Player) -> usize {
        self.nodes.clear();

        let root = Node::new();
        self.nodes.push(root);

        for _ in 0..MAX_ITERATIONS {
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
                    break;
                }

                current_player = current_player.next();
            }

            //EXPANSION
            if self.nodes[current_node_index].can_expand() {
                current_node_index = self.expand(current_node_index, &board, current_player);
                current_player = current_player.next();
            }

            //SIMULATION
            let winner = self.rollout(&board, current_player);

            //BACKPROPOGATION
            self.backpropogate(current_node_index, winner);
        }

        self.nodes
            .iter()
            .max_by(|a, b| a.visits.cmp(&b.visits))
            .expect("nodes should not be empty")
            .game_move
            .expect("node should have move")
    }
}
