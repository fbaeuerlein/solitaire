use super::board::{Board, Move};

pub struct Game
{
    initial_board: Board,
}

impl Game {
    pub fn from_string(s : &str) -> Option<Self>
    {
        Board::from_string(s).map(|b| Game{initial_board : b})
    }

    pub fn play(&self) -> Option<Vec<Move>>
    {
        // println!("Starting with\n=====================================");
        // self.initial_board.print();
        let moves = self.initial_board.get_all_possible_moves();
        let mut result = Vec::new();
        for m in moves
        {
            let v = self.solve(&self.initial_board, &m);
            if !v.is_empty()
            {
                result = v;
                result.reverse();
                break;
            }
  
        }
        match result.is_empty() {
            true => None,
            false => Some(result)
        }
    }

    pub fn print(&self, moves: &Vec<Move>)
    {
        println!("Starting with: ");
        self.initial_board.print();

        let mut b = self.initial_board.clone();
        for m in moves
        {
            println!("====================================================");
            println!("{:?} => ", m);
            b.apply_move(m);
            b.print();
        }
    }

    fn solve(&self, b : &Board, next_move : &Move ) -> Vec<Move>
    {
        let mut n = b.clone();
        // println!("{:?}\n=====================================", next_move);
        // b.print();
        n.apply_move(next_move);

        if n.is_finished()
        {
            vec![*next_move]
        }
        else 
        {    
            let moves = n.get_all_possible_moves();
            if moves.is_empty()
            {
                vec![]
            }
            else
            { 
                // moves.iter().map(|m| self.solve(&n, m)).any(|v| v)
                for m in moves
                {
                    let mut v = self.solve(&n, &m);
                    if !v.is_empty()
                    {
                        v.push(*next_move);
                        return v;
                    }
                }
                vec![]
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
                      O O O O
                        O O";
        let g = Game::from_string(s).unwrap();

        let moves = g.play().unwrap();

        assert!(moves.contains(&Move::new( (3, 1),(1, 1))));
    }

    #[test]
    fn play_game_02()
    {
        let s =  "X X
                      X X X X
                      X X O X
                        X X";
        let g = Game::from_string(s).unwrap();

        let m = g.play();

        println!("Moves: {:?}", m);
    }

    #[test]
    fn play_game_03()
    {
        let s =  "O O
                      O O X X
                      O X O O
                        O O";
        let g = Game::from_string(s).unwrap();

        let moves = g.play().unwrap();
        assert!(moves.contains(&Move::new( (3, 1), (1, 1))));
        assert!(moves.contains(&Move::new( (1, 1), (1, 3))));
    }

    #[test]
    fn play_game_04()
    {
        let s =  "X X X
                        X X X
                    X X X X X X X 
                    X X X O X X X
                    X X X X X X X
                        X X X
                        X X X";

        let g = Game::from_string(s).unwrap();

        let m = g.play();

        g.print(&m.unwrap());
    }
}