use crate::board::Board;




pub enum Player{
    X,O
}


pub trait Move{

    fn compute_move(&self , game_board :&Board, player:Player)->usize;

}
