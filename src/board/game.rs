use super::board::{Board, Move};
//use std::rc::Rc;

pub struct Game
{
    initial_board: Board,
}

impl Game {
    pub fn from_string(s : &str) -> Option<Self>
    {
        Board::from_string(s).map(|b| Game{initial_board : b})
    }

    pub fn play(&self)
    {
        println!("Starting with\n=====================================");
        self.initial_board.print();
        let moves = self.initial_board.get_all_possible_moves();
        for m in moves
        {
            self.solve(&self.initial_board, &m);
        }
    }

    fn solve(&self, b : &Board, next_move : &Move ) -> Option<Move>
    {
        let mut n = b.clone();
        println!("{:?}\n=====================================", next_move);
        b.print();
        n.apply_move(next_move);

        if b.is_finished()
        {
            Some(*next_move)
        }
        else 
        {    
            let moves = n.get_all_possible_moves();
            if moves.is_empty()
            {
                None
            }
            else
            { 
                moves.iter().map(|m| self.solve(&n, m)).find(|m| m.is_some()).flatten()
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_game_01()
    {
        let s =  "O O
                      O O X X
                      O O O X
                        O X";
        let g = Game::from_string(s).unwrap();

        g.play();   
    }

    #[test]
    fn play_game_02()
    {
        let s =  "X X
                      X X X X
                      X X O X
                        X X";
        let g = Game::from_string(s).unwrap();

        g.play();   
    }
}