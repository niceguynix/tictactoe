mod ai;
mod board;
mod minimax;

use ai::Move;
use ai::{Player, PlayerType};
use board::Board;
use minimax::MiniMax;
use std::io::{self, Write};
use std::{fmt::Debug, str::FromStr};

fn main() {
    play();
}

fn play() {
    println!("1.Player vs Player");
    println!("2.Player vs Computer");
    print!("\nEnter Choice: ");
    let ch: i32 = get_user_input();

    let mut ch2 = ' ';
    if ch == 2 {
        print!("\nWould you like to play first(y/n): ");
        ch2 = get_user_input();
        match ['y', 'n'].contains(&ch2) {
            true => (),
            false => panic!("Wrong input"),
        }
    }
    let player1 = match (ch, ch2) {
        (2, 'n') => Player::new(PlayerType::C, None),
        (n, _) => Player::new(PlayerType::H, Some(1)),
        (_, _) => panic!("Invalid Input"),
    };

    let player2 = match (ch, ch2) {
        (2, 'y') => Player::new(PlayerType::C, None),
        (n, _) => Player::new(PlayerType::H, Some(2)),
        (_, _) => panic!("Invalid Input"),
    };

    game_loop(player1, player2);
}

fn game_loop(p1: Player, p2: Player) {
    let mut game_board = Board::new();
    let mut curr_player = true;
    let minimax = MiniMax::new();

    for _i in 1..10 {
        println!("\n{}\n", game_board);
        let player = {
            if curr_player {
                &p1
            } else {
                &p2
            }
        };
        let n: usize = match player.ptype {
            PlayerType::C => minimax.compute_move(&game_board, curr_player),
            PlayerType::H => {
                print!("\nEnter Choice: ");
                get_user_input()
            }
        };

        let (i, j) = convert_input_to_pos(n);

        match (curr_player, game_board.board[i][j]) {
            (true, ' ') => game_board.board[i][j] = 'X',
            (false, ' ') => game_board.board[i][j] = 'O',
            (_, _) => panic!("Position already filled"),
        }

        if game_board.check_win() {
            break;
        }

        curr_player = !curr_player;
    }

    println!("\n{}\n", game_board);
    println!("{} won!", if curr_player { p1 } else { p2 })
}

fn get_user_input<T: FromStr>() -> T {
    io::stdout().flush().unwrap();
    let mut s = String::with_capacity(2);
    io::stdin().read_line(&mut s).expect("User input Failed");
    match s.trim().parse() {
        Ok(n) => n,
        Err(err) => panic!("Wrong input!"),
    }
}

fn convert_input_to_pos(n: usize) -> (usize, usize) {
    let i = (n - 1) / 3;
    let j = (n - 1) % 3;

    (i, j)
}
