use crate::ai::{Move, PlayerType};
use crate::board::{self, Board};

pub struct MiniMax {}

impl MiniMax {
    pub fn new() -> Self {
        Self {}
    }
}

impl MiniMax {
    fn minimax_compute(
        game_board: &mut Board,
        maximize: bool,
        player_sign: char,
        enemy_sign: char,
    ) -> i32 {
        if game_board.check_win() {
            if maximize {
                return -1;
            } else {
                return 1;
            }
        }

        if game_board.check_draw() {
            return 0;
        }

        if maximize {
            let mut max_value = -999999;
            for i in 0..3 as usize {
                for j in 0..3 as usize {
                    if game_board.board[i][j] == ' ' {
                        game_board.board[i][j] = player_sign;
                        max_value = std::cmp::max(
                            max_value,
                            Self::minimax_compute(game_board, false, player_sign, enemy_sign),
                        );
                        game_board.board[i][j] = ' ';
                    }
                }
            }

            max_value
        } else {
            let mut min_value = 999999;
            for i in 0..3 as usize {
                for j in 0..3 as usize {
                    if game_board.board[i][j] == ' ' {
                        game_board.board[i][j] = enemy_sign;
                        min_value = std::cmp::min(
                            min_value,
                            Self::minimax_compute(game_board, true, player_sign, enemy_sign),
                        );
                        game_board.board[i][j] = ' ';
                    }
                }
            }

            min_value
        }
    }
}

impl Move for MiniMax {
    fn compute_move(&self, game_board: &Board, player_sign: bool) -> usize {
        let mut t = game_board.clone();

        let mut n = 1;
        let mut max_value = -99999;

        let player_sign2 = match player_sign {
            true => 'X',
            false => 'O',
        };

        let enemy_sign = match player_sign {
            true => 'O',
            false => 'X',
        };

        for i in 0..3 as usize {
            for j in 0..3 as usize {
                if t.board[i][j] == ' ' {
                    t.board[i][j] = player_sign2;

                    let temp = Self::minimax_compute(&mut t, false, player_sign2, enemy_sign);
                    if temp > max_value {
                        max_value = temp;
                        n = i * 3 + j + 1;
                    }
                    t.board[i][j] = ' ';
                }
            }
        }

        n
    }
}
