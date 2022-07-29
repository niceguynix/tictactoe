use std::fmt::{Display, Formatter, Result};

use crate::board::Board;

pub enum PlayerType {
    H,
    C,
}

pub struct Player {
    pub ptype: PlayerType,
    //GoFirst: i32,
    no: Option<i32>,
}

impl Player {
    pub fn new(player: PlayerType, no: Option<i32>) -> Self {
        Player {
            ptype: (player),
            no,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.ptype {
            PlayerType::C => write!(f, "Computer"),
            PlayerType::H => match self.no {
                Some(n) => write!(f, "Player {}", n),
                None => write!(f, "Player"),
            },
        }
    }
}

pub trait Move {
    fn compute_move(&self, game_board: &Board, player_sign: bool) -> usize;
}
