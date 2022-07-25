mod ai;
mod board;
mod minimax;

use std::io::{self, Write};

use ai::Move;

fn main() {
    play_vs_ai();
}

fn play_vs_ai() {
    let mut game_board = board::Board::new();
    let mut player_turn = true;
    let mut player_won = false;
    let ai = minimax::MiniMax::new();

    for _i in 1..10 {
        println!("{}", game_board);
        let n = if _i % 2 == 1 {
            get_user_input(player_turn)
        } else {
            ai.compute_move(&game_board, ai::Player::O)
        };
        let (i, j) = convert_input_to_pos(n);

        if game_board.board[i][j] != ' ' {
            panic!("Position is already filled");
        }

        game_board.board[i][j] = if player_turn { 'X' } else { 'O' };

        if game_board.check_win() {
            player_won = true;
            break;
        }
        player_turn = !player_turn;
    }
    println!("{}", game_board);
    if player_won {
        println!("Player {} won", {
            if player_turn {
                1
            } else {
                2
            }
        });
    } else {
        println!("Draw!");
    }
}

fn play() {
    let mut game_board = board::Board::new();
    let mut player_turn = true;
    let mut player_won = false;

    for _i in 1..10 {
        println!("{}", game_board);
        let n = get_user_input(player_turn);
        let (i, j) = convert_input_to_pos(n);

        if game_board.board[i][j] != ' ' {
            panic!("Position is already filled");
        }

        game_board.board[i][j] = if player_turn { 'X' } else { 'O' };

        if game_board.check_win() {
            player_won = true;
            break;
        }
        player_turn = !player_turn;
    }
    if player_won {
        println!("Player {} won", {
            if player_turn {
                1
            } else {
                2
            }
        });
    } else {
        println!("Draw!");
    }
}

fn get_user_input(player_turn: bool) -> usize {
    print!("Player {} Choice :", {
        if player_turn {
            1
        } else {
            2
        }
    });
    io::stdout().flush().unwrap();
    let mut s = String::with_capacity(2);
    io::stdin().read_line(&mut s).expect("User input Failed");
    s.trim().parse().expect("Input is not a number")
}

fn convert_input_to_pos(n: usize) -> (usize, usize) {
    let i = (n - 1) / 3;
    let j = (n - 1) % 3;

    (i, j)
}
