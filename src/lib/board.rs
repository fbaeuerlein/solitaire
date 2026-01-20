// pub mod board_manager;
// use board_manager::BoardManager;
use super::types::{Coordinate, Coordinates};
use super::board_manager::BoardManager;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    start: Coordinates,
    end: Coordinates,
}

impl Move
{
    pub fn new(start: Coordinates, end: Coordinates) -> Self
    {
        Move { start, end }
    }
}

#[derive(Clone)]
pub struct Board
{
    pos : Vec<bool>,
    mgr : Rc<BoardManager>
}
/*
    0 1 2 3 4 5 6
  0     * * *      +
  1     * * *      + = n - 1
  2 * * * * * * *  +
  3 * * * * * * *  | = n
  4 * * * * * * *  +
  5     * * *
  6     * * *

*/
impl Board {

    pub fn new(board_mgr : Rc<BoardManager>) -> Self
    {
        let n =  board_mgr.n();
        Board { pos: vec![false; BoardManager::array_size(n) ], mgr: board_mgr}
    }

    pub fn count_pegs(&self) -> usize
    {
        self.pos.iter().filter(|p| **p).count()
    }

    pub fn is_finished(&self) -> bool
    {
        [0, 1].contains(&self.count_pegs())
    }

    pub fn from_string(s: &str) -> Option<Board>
    {
        let without_spaces : String = s.chars().filter(|c| !c.is_whitespace() && (*c == 'O' || *c == 'X')).collect();

        if let Some(n) = BoardManager::n_from_array_size(without_spaces.len())
        {
            let v : Vec<bool> = without_spaces.chars().map(|c| match c { 'X' => true, _ => false }).collect();
            let mgr = Rc::new(BoardManager::new(n));
            return Some(Board{ pos : v, mgr : mgr});
        }
        None
    }

    pub fn get_board_manager(&self) -> Rc<BoardManager>
    {
        self.mgr.clone()
    }

    fn get(&self, coords : &Coordinates) -> Option<bool>
    {
        match self.mgr.get_index(coords) {
            Some(idx) => Some(self.pos[idx]),
            _ => None
        }
    }

    pub fn set(&mut self, coords : &Coordinates, value: bool) -> bool
    {
        if let Some(idx) = self.mgr.get_index(coords)
        {
            self.pos[idx] = value;
            return true;
        }    

        false
    }

    pub fn set_all(&mut self, value: bool)
    {
        self.pos.iter_mut().for_each(|v| *v = value);
    }

    pub fn print(&self)
    {
        println!("{}", self.into_string());
    }

    pub fn into_string(&self) -> String
    {
        let mut result = String::new();
        let size = self.mgr.size();

        for y in 0..size as Coordinate // lines
        {
            for x in 0..size as Coordinate // cols
            {
                match self.get(&(x, y))
                {
                    Some(v) => if v {result.push('X'); } else { result.push('O'); },
                    None => result.push(' ')
                }
                result.push(' ');
            }
            result.push('\n');
        }

        result

    }

    fn check_left_neighbor(&self, coords : &Coordinates) -> Option<bool>
    {
        match coords.0
        {
            x if coords.0 > 0 => self.get(&(coords.0 - 1, coords.1)),
            _ => None
        }
    }

    fn check_right_neighbor(&self, coords : &Coordinates) -> Option<bool>
    {
        match coords.0
        {
            x if (coords.0 as usize) < (self.mgr.size() - 1) => self.get(&(coords.0 + 1, coords.1)),
            _ => None
        }
    }

    fn check_upper_neighbor(&self, coords : &Coordinates) -> Option<bool>
    {
        match coords.1
        {
            x if coords.1 > 0 => self.get(&(coords.0, coords.1 - 1)),
            _ => None
        }
    }

    fn check_lower_neighbor(&self, coords : &Coordinates) -> Option<bool>
    {
        match coords.1
        {
            x if (coords.1 as usize) < (self.mgr.size() - 1) => self.get(&(coords.0, coords.1 + 1)),
            _ => None
        }
    }

    pub fn get_possible_moves(&self, coords: &Coordinates) -> Vec<Coordinates>
    {
        let mut result = vec![];
        let size = self.mgr.size();

        if let Some(v) = self.get(coords) && v // is there really a stick?
        {
            if Some(true) == self.check_left_neighbor(coords) && coords.0 > 1
            {
                let is_empty_position = |x : &u8| Some(false) == self.get(&(*x, coords.1));
                let add_to_result = |x| result.push((x, coords.1));
                (0..coords.0 - 1).rev().find(is_empty_position).map(add_to_result);
            }

            if Some(true) == self.check_right_neighbor(coords) && (coords.0 as usize) < (size - 1)
            {
                let is_empty_position = |x : &u8| Some(false) == self.get(&(*x, coords.1));
                let add_to_result = |x| result.push((x, coords.1));
                (coords.0 + 1..size as u8).find(is_empty_position).map(add_to_result);
            }

            if Some(true) == self.check_upper_neighbor(coords) && coords.1 > 1
            {
                let is_empty_position = |y : &u8| Some(false) == self.get(&(coords.0, *y));
                let add_to_result = |y| result.push((coords.0, y));
                (0..coords.1 - 1).rev().find(is_empty_position).map(add_to_result);
            }

            if Some(true) == self.check_lower_neighbor(coords) && (coords.1 as usize) < (size - 1)
            {
                let is_empty_position = |y : &u8| Some(false) == self.get(&(coords.0, *y));
                let add_to_result = |y| result.push((coords.0, y));
                (coords.1 + 1..size as u8).find(is_empty_position).map(add_to_result);
            }
        }

        result
    }

    pub fn get_all_possible_moves(&self) -> Vec<Move>
    {
        let start_coords : Vec<_> = 
            self.pos.iter().enumerate().filter_map(|(i, peg)| 
                match peg 
                { 
                    &true => self.mgr.get_coordinate(i), 
                    &false => None 
                }).collect();

        let mut result = Vec::new();

        for start in start_coords
        {
            for end in self.get_possible_moves(start)
            {
                result.push(Move{ start : *start, end : end});
            }
        }

        result
    }

    pub fn apply_move(&mut self, m: &Move)
    {
        let dir = ((m.end.0 as i16 - m.start.0 as i16).signum(), (m.end.1 as i16 - m.start.1 as i16).signum());

        let mut current = (m.start.0 as i16, m.start.1 as i16);

        while current != (m.end.0 as i16, m.end.1 as i16)
        {
            self.set(&(current.0 as Coordinate, current.1 as Coordinate ), false);
            current.0 += dir.0;
            current.1 += dir.1;
        } 

        self.set(&m.end, true);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn from_string_is_successful()
    {
        let s =  "O O
                      O O X X
                      O O O X
                        O X";

        let b = Board::from_string(s).unwrap();

        assert_eq!(b.get_board_manager().n(), 2);
        assert_eq!(b.get(&(1, 0)), Some(false));
        assert_eq!(b.get(&(2, 0)), Some(false));

        assert_eq!(b.get(&(0, 1)), Some(false));
        assert_eq!(b.get(&(1, 1)), Some(false));
        assert_eq!(b.get(&(2, 1)), Some(true));
        assert_eq!(b.get(&(3, 1)), Some(true));

        assert_eq!(b.get(&(0, 2)), Some(false));
        assert_eq!(b.get(&(1, 2)), Some(false));
        assert_eq!(b.get(&(2, 2)), Some(false));
        assert_eq!(b.get(&(3, 2)), Some(true));

        assert_eq!(b.get(&(1, 3)), Some(false));
        assert_eq!(b.get(&(2, 3)), Some(true));
    }

        #[test]
    fn to_string_is_successful()
    {
        let s =  
"  O O   
O O X X 
O O O X 
  O X   
";

        let b = Board::from_string(s).unwrap();
        assert_eq!(b.into_string(), s);
    }

    #[test]
    fn count_pegs_is_successfull()
    {
        let s =  "O O
                      O O X X
                      O O O X
                        O X";

        let mut b = Board::from_string(s).unwrap();
        assert_eq!(b.count_pegs(), 4);
        assert_eq!(b.is_finished(), false);

        b.set_all(true);
        assert_eq!(b.count_pegs(), 12);

        b.set_all(false);
        assert_eq!(b.count_pegs(), 0);
        assert_eq!(b.is_finished(), true);

        b.set(&(1, 1), true);
        assert_eq!(b.is_finished(), true);

    }

    #[test]
    fn new_initializes_all_positions_false() {
        let mgr = Rc::new(BoardManager::new(3));
        let b = Board::new(mgr);
        for (i, &p) in b.pos.iter().enumerate() {
            assert!(!p, "pos[{}] should be false on new board", i);
        }
    }

    #[test]
    fn apply_move_01() {
        let mgr = Rc::new(BoardManager::new(2));
        let mut b = Board::new(mgr);
        b.set_all(true);

        b.set(&(1, 0), false);
        b.set(&(1, 1), false);
        b.set(&(0, 2), false);

        b.apply_move(&Move{start: (3, 1), end: (1, 1)});

        assert_eq!(b.get(&(1,0)).unwrap(), false);
        assert_eq!(b.get(&(0,2)).unwrap(), false);
        assert_eq!(b.get(&(3,1)).unwrap(), false);
        assert_eq!(b.get(&(1,1)).unwrap(), true);
    }

    #[test]
    fn apply_move_horizontal() {
        let mgr = Rc::new(BoardManager::new(3));
        let mut b = Board::new(mgr);

        // Set up: (4,3) has a peg, (3,3) is empty, (2,3) has a peg
        b.set(&(0, 2), true);
        b.set(&(0, 3), true);

        b.apply_move(&Move{start: (0, 2), end: (0, 4)});
        
        assert_eq!(b.get(&(0, 3)).unwrap(), false, "(0, 3) should be false");

        for y in 0..=5
        {
            b.set(&(4, y), true);
        }
        b.apply_move(&Move{start: (4, 0), end: (4, 6)});
        assert!((0..=5).into_iter().map(|y| b.get(&(4, y)).unwrap() ).all(|b| b == false), "should be false");
    }

    #[test]
    fn apply_move_vertical() {
        let mgr = Rc::new(BoardManager::new(3));
        let mut b = Board::new(mgr);

        // Set up: (4,3) has a peg, (3,3) is empty, (2,3) has a peg
        b.set(&(2, 0), true);
        b.set(&(3, 0), true);
        b.apply_move(&Move{start: (2, 0), end: (4, 0)});
        
        assert_eq!(b.get(&(3, 0)).unwrap(), false, "(3, 0) should be false");

        for x in 0..=5
        {
            b.set(&(x, 4), true);
        }
        b.apply_move(&Move{start: (0, 4), end: (6, 4)});
        assert!((0..=5).into_iter().map(|x| b.get(&(x, 4)).unwrap() ).all(|b| b == false), "should be false");
    }

    #[test]
    fn valid_move_horizontal() {
        let mgr = Rc::new(BoardManager::new(3));
        let mut b = Board::new(mgr);

        // Set up: (4,3) has a peg, (3,3) is empty, (2,3) has a peg
        b.set(&(2, 3), true);
        b.set(&(1, 3), true);
        b.set(&(3, 3), true); // this is were we start
        b.set(&(4, 3), true);
        b.set(&(5, 3), true);
        
        let moves = b.get_possible_moves(&(3, 3));
        assert!(moves.contains(&(0, 3)), "Should be able to move left to (0 ,3) but {:?}", moves);
        assert!(moves.contains(&(6, 3)), "Should be able to move right to (6 ,3) but {:?}", moves);
    }

    #[test]
    fn valid_move_vertical() {
        let mgr = Rc::new(BoardManager::new(3));
        let mut b = Board::new(mgr);

        // Set up: (4,3) has a peg, (3,3) is empty, (2,3) has a peg
        b.set(&(3, 2), true);
        b.set(&(3, 1), true);
        b.set(&(3, 3), true); // this is were we start
        b.set(&(3, 4), true);
        b.set(&(3, 5), true);
        
        let moves = b.get_possible_moves(&(3, 3));
        assert!(moves.contains(&(3, 0)), "Should be able to move left to (3, 0) but {:?}", moves);
        assert!(moves.contains(&(3, 6)), "Should be able to move right to (3, 6) but {:?}", moves);
    }
        #[test]
    fn valid_move_to_left_from_position_for_multiple_empty_spaces() {
        let mgr = Rc::new(BoardManager::new(3));
        let mut b = Board::new(mgr);

        b.set(&(4, 3), true); // this is were we start
        b.set(&(3, 3), true); 
        b.set(&(2, 3), false);
        b.set(&(1, 3), true);
        b.set(&(0, 3), false);
        
        let moves = b.get_possible_moves(&(4, 3));
        assert!(moves.contains(&(2, 3)), "Should be able to move left to (2 ,3)");
    }

    #[test]
    fn moves_left_no_empty_space() {
        let mgr = Rc::new(BoardManager::new(3));
        let mut b = Board::new(mgr);
        b.set_all(true);
        b.set(&(3, 3), false);


        b.set(&(3, 3), true);
        b.set(&(2, 3), true);
        b.set(&(1, 3), true);
        b.set(&(3, 3), true);

        let moves = b.get_possible_moves(&(3, 3));
        assert!(moves.is_empty(), "Should have no moves when no empty space to the left");
    }


 
    #[test]
    fn state_returns_some_for_valid_coords() {
        let mgr = Rc::new(BoardManager::new(3));
        let mut b = Board::new(mgr);

        let coords_index = [
            ((2, 0), 0),
            ((3, 0), 1),
            ((4, 0), 2),
            ((2, 1), 3),
            ((3, 1), 4),
            ((4, 1), 5),

            ((0, 2), 6),
            ((1, 2), 7),
            ((2, 2), 8),
            ((3, 2), 9),
            ((4, 2), 10),
            ((5, 2), 11),
            ((6, 2), 12),

            ((0, 3), 13),
            ((1, 3), 14),
            ((2, 3), 15),
            ((3, 3), 16),
            ((4, 3), 17),
            ((5, 3), 18),
            ((6, 3), 19),

            ((0, 4), 20),
            ((1, 4), 21),
            ((2, 4), 22),
            ((3, 4), 23),
            ((4, 4), 24),
            ((5, 4), 25),
            ((6, 4), 26),

            ((2, 5), 27),
            ((3, 5), 28),
            ((4, 5), 29),
            ((2, 6), 30),
            ((3, 6), 31),
            ((4, 6), 32)
        ];

        for (coords, i) in coords_index
        {
            b.pos[i] = true;
            assert_eq!(b.get(&coords), Some(true));
            b.pos[i] = false;
            assert_eq!(b.get(&coords), Some(false));
        }

    }

    #[test]
    fn state_returns_none_for_invalid_coords() {
        let mgr = Rc::new(BoardManager::new(3));
        let b = Board::new(mgr);

        assert_eq!(b.get(&(0, 0)), None);
        assert_eq!(b.get(&(1, 0)), None);
        assert_eq!(b.get(&(5, 0)), None);
        assert_eq!(b.get(&(6, 0)), None);
        assert_eq!(b.get(&(0, 1)), None);
        assert_eq!(b.get(&(1, 1)), None);
        assert_eq!(b.get(&(5, 1)), None);
        assert_eq!(b.get(&(6, 1)), None);

        assert_eq!(b.get(&(0, 5)), None);
        assert_eq!(b.get(&(1, 5)), None);
        assert_eq!(b.get(&(5, 5)), None);
        assert_eq!(b.get(&(6, 5)), None);
        assert_eq!(b.get(&(0, 6)), None);
        assert_eq!(b.get(&(1, 6)), None);
        assert_eq!(b.get(&(5, 6)), None);
        assert_eq!(b.get(&(6, 6)), None);        
    }

    #[test]
    fn possible_moves_01()
    {
        let s =  "O O
                      O O O O
                      O X O X
                        X X";

        let b = Board::from_string(s).unwrap();
        let m = b.get_all_possible_moves();
        assert_eq!(m.len(), 1);
        assert!(m.contains(&Move { start: (1, 3), end: (1, 1) }));
    }

    #[test]
    fn possible_moves_02()
    {
        let s = 
        "X X
       X X X X
       X X O X
         X X";

        let b = Board::from_string(s).unwrap();
        let m = b.get_all_possible_moves();
        assert_eq!(m.len(), 2);
        assert!(m.contains(&Move { start: (2, 0), end: (2, 2) }), "{:?}", m);
        assert!(m.contains(&Move { start: (0, 2), end: (2, 2) }), "{:?}", m);
    }

}

