
mod board;
use board::board::Board;
use board::board_manager::BoardManager;
use std::rc::Rc;

use crate::board::board::Move;
use crate::board::types::Coordinates;


fn solve(b : &Board, m : &Move) 
{
    let mut n = b.clone();
    n.apply_move(m);
    // println!("================================");
    // println!("{:?}", m);
    // n.print();
    let moves = n.get_all_possible_moves();
    for next_move in moves
    {
        solve(&n, &next_move);
    }
}

fn main() {
    let mgr = Rc::new(BoardManager::new(3));
    let mut b = Board::new(mgr);

    b.set_all(true);
    b.set(&(3, 3), false);

    let moves = b.get_all_possible_moves();
    println!("{:?}", moves);

    b.print();

    solve(&b, &moves[0]);
}
