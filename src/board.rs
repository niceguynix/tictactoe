use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub struct Board {
    pub board: [[char; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']],
        }
    }
}

impl Board {
    pub fn check_win(&self) -> bool {
        for i in self.board.into_iter() {
            if i[0] == i[1] && i[1] == i[2] && i[0] != ' ' {
                return true;
            }
        }

        for j in 0..=2 {
            if self.board[0][j] == self.board[1][j]
                && self.board[1][j] == self.board[2][j]
                && self.board[2][j] != ' '
            {
                return true;
            }
        }

        if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
            && self.board[2][2] != ' '
        {
            return true;
        }

        if self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
            && self.board[1][1] != ' '
        {
            return true;
        }

        false
    }
}

impl Board {
    pub fn check_draw(&self) -> bool {
        for i in 0..3 as usize {
            for j in 0..3 as usize {
                if self.board[i][j] == ' ' {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let t = self.board;
        writeln!(
            f,
            "\n{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            t[0][0], t[0][1], t[0][2], t[1][0], t[1][1], t[1][2], t[2][0], t[2][1], t[2][2]
        )
    }
}
