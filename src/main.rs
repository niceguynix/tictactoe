mod board;
mod minmax;


use std::io;


fn main() {

        play();
        minmax::MinMax::game_move();
}


fn play(){
    let mut game_board = board::Board::new();
    let mut player_turn = true;
    let mut player_won = false;
    
    for _i in 1..10{
        println!("{}",game_board);
        print!("Player {} Choice :",{if player_turn  {1} else {2}});
        io::stdout().flush().unwrap();
        let mut s = String::with_capacity(2);
        let i:usize;
        let j:usize;
        io::stdin().read_line(&mut s).expect("User input Failed");
        let n:usize = s.trim().parse().expect("Input is not a number");
        i = (n-1)/3;
        j=(n-1)%3;

        
        if game_board.board[i][j] != ' '{
            panic!("Position is already filled");
        }

        game_board.board[i][j] = if player_turn {'X'} else {'O'} ;


        if game_board.check_win(){
            player_won = true;
            break;
        } 
        player_turn = !player_turn;
    }
    if player_won {
        println!("Player {} won",{if player_turn {1} else {2}} );
    }else{
        println!("Draw!");
    }

}
