use std::{error::Error, usize};

use crate::board::Player;

#[derive(strum_macros::Display, Debug)]
pub enum GameError {
    InvalidMove,
}

impl Error for GameError {}

#[derive(Clone)]

pub struct Board {
    n: usize,
    k: usize,
    cells: Vec<Option<Player>>,
    last_move: usize,
}

struct Dir {
    dx: i8,
    dy: i8,
}

const DIRECTIONS: [Dir; 4] = [
    Dir { dx: 1, dy: 0 },
    Dir { dx: 0, dy: 1 },
    Dir { dx: 1, dy: 1 },
    Dir { dx: 1, dy: -1 },
];

impl Board {
    pub fn new(n: usize, k: usize) -> Self {
        Self {
            n: n,
            k: k,
            cells: vec![None; n * n],
            last_move: 0,
        }
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

            if (i + 1) % self.n == 0 {
                print!("\n")
            }
        }

        print!("\n")
    }

    pub fn apply_move(
        &mut self,
        idx: usize,
        player: Player,
    ) -> Result<Option<Player>, Box<dyn Error>> {
        self.make_move(idx, player)?;

        let winner = self.check_winner();
        Ok(winner)
    }

    fn make_move(&mut self, idx: usize, player: Player) -> Result<(), Box<dyn Error>> {
        if self.cells[idx].is_some() {
            return Err(Box::from(GameError::InvalidMove));
        }

        self.cells[idx] = Some(player);
        self.last_move = idx;

        Ok(())
    }

    pub fn check_winner(&self) -> Option<Player> {
        self.check_winner_from(self.last_move)
    }

    pub fn check_winner_from(&self, idx: usize) -> Option<Player> {
        let p = self.cells[idx];
        if p.is_none() {
            return None;
        }

        let x = idx % self.n;
        let y = idx / self.n;

        for d in DIRECTIONS {
            let mut count = 1;

            for dir_mod in [-1, 1] {
                let mut nx = (x as i8) + d.dx * dir_mod;
                let mut ny = (y as i8) + d.dy * dir_mod;

                while nx >= 0 && ny >= 0 && (nx as usize) < self.n && (ny as usize) < self.n {
                    let n_idx = (ny as usize) * self.n + (nx as usize);
                    if self.cells[n_idx] != p {
                        break;
                    }

                    count += 1;
                    if count >= self.k {
                        return p;
                    }

                    nx += d.dx * dir_mod;
                    ny += d.dy * dir_mod;
                }
            }
        }

        None
    }
}
