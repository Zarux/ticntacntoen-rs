use std::error::Error;

use crate::board::Player;

#[derive(strum_macros::Display, Debug)]
pub enum GameError {
    InvalidMove,
}

impl Error for GameError {}

#[derive(Clone)]

pub struct Board {
    n: i16,
    k: i8,
    cells: Vec<Option<Player>>,
    last_move: Option<i16>,
    x_list: Vec<i16>,
    y_list: Vec<i16>,
}

struct Dir {
    dx: i16,
    dy: i16,
}

const DIRECTIONS: [Dir; 4] = [
    Dir { dx: 1, dy: 0 },
    Dir { dx: 0, dy: 1 },
    Dir { dx: 1, dy: 1 },
    Dir { dx: 1, dy: -1 },
];

impl Board {
    pub fn new(n: i16, k: i8) -> Self {
        Board::new_from_state(n, k, vec![None; (n * n) as usize])
    }

    pub fn new_from_state(n: i16, k: i8, cells: Vec<Option<Player>>) -> Self {
        if cells.len() != (n * n) as usize {
            panic!("invalid state loaded")
        }

        let x_list: Vec<i16> = (0..n * n).map(|i| i % n).collect();
        let y_list: Vec<i16> = (0..n * n).map(|i| i / n).collect();
        Self {
            n: n,
            k: k,
            cells: cells,
            last_move: None,
            x_list: x_list,
            y_list: y_list,
        }
    }

    pub fn cells(&self) -> &[Option<Player>] {
        &self.cells
    }

    pub fn n(&self) -> i16 {
        self.n
    }

    pub fn is_tie(&self) -> bool {
        !self.cells.iter().any(|e| e.is_none())
    }

    pub fn print(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            print!(
                "[{}]",
                match cell {
                    Some(cell) => cell.to_string(),
                    None => " ".to_string(),
                }
            );

            if (i + 1) % (self.n as usize) == 0 {
                print!("\n")
            }
        }

        print!("\n")
    }

    pub fn apply_move(&mut self, m: i16, player: Player) -> Result<Option<Player>, Box<dyn Error>> {
        self.make_move(m, player)?;

        let winner = self.check_winner();
        Ok(winner)
    }

    pub fn undo_last_move(&mut self) -> Result<(), Box<dyn Error>> {
        let last_move = match self.last_move {
            Some(m) => m,
            None => return Err(Box::from(GameError::InvalidMove)),
        };

        self.cells[last_move as usize] = None;
        self.last_move = None;

        Ok(())
    }

    fn make_move(&mut self, m: i16, player: Player) -> Result<(), Box<dyn Error>> {
        if self.cells[m as usize].is_some() {
            return Err(Box::from(GameError::InvalidMove));
        }

        self.cells[m as usize] = Some(player);
        self.last_move = Some(m);

        Ok(())
    }

    pub fn check_winner(&self) -> Option<Player> {
        let last_move = match self.last_move {
            Some(m) => m,
            None => return None,
        };

        self.check_winner_from(last_move)
    }

    pub fn check_winner_from(&self, m: i16) -> Option<Player> {
        let p = self.cells[m as usize];
        if p.is_none() {
            return None;
        }

        let x: i16;
        let y: i16;
        unsafe {
            x = *self.x_list.get_unchecked(m as usize);
            y = *self.y_list.get_unchecked(m as usize);
        }

        for d in DIRECTIONS {
            let mut count = 1;

            for dir_mod in [-1, 1] {
                let dx_dir_mod = d.dx * dir_mod;
                let dy_dir_mod = d.dy * dir_mod;

                let mut nx = x + dx_dir_mod;
                let mut ny = y + dy_dir_mod;

                while nx >= 0 && ny >= 0 && nx < self.n && ny < self.n {
                    let n_idx = ny * self.n + nx;
                    unsafe {
                        if *self.cells.get_unchecked(n_idx as usize) != p {
                            break;
                        }
                    }

                    count += 1;
                    if count >= self.k {
                        return p;
                    }

                    nx += dx_dir_mod;
                    ny += dy_dir_mod;
                }
            }
        }

        None
    }
}

#[allow(dead_code)]
pub fn from_board_string_to_state(board: &str) -> Vec<Option<Player>> {
    let mut cells = vec![];
    let mut prev_c = ' ';
    let mut board_size = 0;
    for c in board.trim().chars() {
        if board_size == 0 && !cells.is_empty() && c == '\n' {
            board_size = cells.len();
        }

        if c == ' ' && prev_c == '[' {
            cells.push(None);
        } else if c == 'X' {
            cells.push(Some(Player::X));
        } else if c == 'O' {
            cells.push(Some(Player::O));
        } else if !['[', ']', ' ', '\n'].contains(&c) {
            println!("character '{c}' invalid");
            panic!("invalid character encountered")
        }
        prev_c = c;
    }

    if cells.len() != board_size * board_size {
        panic!("boardsize inconsistent")
    }

    cells
}

#[allow(dead_code)]
pub fn from_cell_string_to_state(cell_string: &str) -> Vec<Option<Player>> {
    let cells: Vec<Option<Player>> = cell_string
        .trim()
        .chars()
        .map(|c| match c {
            'X' => Some(Player::X),
            'O' => Some(Player::O),
            '_' => None,
            _ => {
                println!("character '{c}' invalid");
                panic!("invalid character encountered")
            }
        })
        .collect();

    if (cells.len() as f64).sqrt().fract() != 0.0 {
        panic!("boardsize inconsistent")
    }

    cells
}
