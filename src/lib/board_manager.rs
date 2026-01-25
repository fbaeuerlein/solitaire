
use num_integer::Roots;

use super::types::{Coordinates, Index};
use std::{collections::HashMap};

#[derive(Clone)]
pub struct BoardManager
{
    n : u8,
    size : usize, 
    i_to_c_map : Vec<Coordinates>, 
    c_to_i_map : HashMap<Coordinates, Index>
}

impl BoardManager
{
    pub fn new(n : u8) -> BoardManager
    {
        let size = match n
        {
            n if n == 0 => 0,
            _ => (3 * n) as usize - 2
        } ;

        let array_size = Self::array_size(n);
        let mut i2c : Vec<Coordinates> = Vec::new();
        let mut c2i : HashMap<Coordinates, usize> = HashMap::with_capacity(array_size);

        // upper 
        for y in 0..(n - 1)
        {
            for x in (n - 1)..(2*n - 1)
            {
                let i = i2c.len();
                i2c.push((x, y));
                c2i.insert((x, y), i);
            }
        }


        // middle 
        for y in (n - 1)..(2*n - 1)
        {
            for x in 0..size as u8
            {
                let i = i2c.len();
                i2c.push((x, y));
                c2i.insert((x, y), i);
            }
        }

        // lower 
        for y in (2 * n - 1)..(3 * n - 2)
        {
            for x in (n - 1)..(2*n - 1)
            {
                let i = i2c.len();
                i2c.push((x, y));
                c2i.insert((x, y), i);
            }
        }

        BoardManager { n : n, size : size, i_to_c_map : i2c, c_to_i_map : c2i }
    }

    pub fn array_size(n : u8) -> usize
    {
        match n
        {
            _ if n == 0 => 0,
            _ => (n as usize * (5 * n as usize - 4)) as usize
        }
    }

    // 0 = 5n^2 -4n - a
    // (4 +/- sqrt(16 + 20a))/ 10 
    pub fn n_from_array_size(a : usize) -> Option<u8>
    {
        let d = 16 + 20 * a;
        let d_sqrt: usize = d.sqrt();

        if d_sqrt.pow(2) == d
        {
            let r = 4 + d_sqrt;
            if r % 10 == 0 
            {
                return Some((r / 10) as u8);
            }
        }
        
        None
    }

    pub fn n(&self) -> u8
    {
        self.n
    }

    pub fn size(&self) -> usize
    {
        self.size
    }

    pub fn get_coordinate(&self, index : usize) -> Option<&Coordinates>
    {
        self.i_to_c_map.get(index)
    }

    pub fn get_index(&self, coords: &Coordinates) -> Option<usize>
    {
        self.c_to_i_map.get(&coords).cloned()
    }

}

mod tests {
    use super::*;

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
    #[test]
    fn get_n_from_array_size_is_working() {

        assert_eq!(BoardManager::n_from_array_size(12), Some(2));
        assert_eq!(BoardManager::n_from_array_size(33), Some(3));
    
        assert_eq!(BoardManager::n_from_array_size(0), None);
    }


    #[test]
    fn n_is_set_correctly() {
        
        for n in 1..10
        {
            let b = BoardManager::new(n);
            assert_eq!(b.n(), n);
        }
    }

    #[test]
    fn size_is_calculated_correctly() {
        let valid_sizes : Vec<(u8, usize)> = vec![(3, 7), (5, 13), (6, 16)];
        for (n, size) in valid_sizes 
        {
            assert_eq!(BoardManager::new(n).size(), size);
        }
    }

    #[test]
    fn get_coordinate_returns_expected_values() {
        let bm = BoardManager::new(3);

        assert_eq!(bm.get_coordinate(0), Some(&(2, 0)));
        assert_eq!(bm.get_coordinate(1), Some(&(3, 0)));
        assert_eq!(bm.get_coordinate(16), Some(&(3,3)));
        assert_eq!(bm.get_coordinate(31), Some(&(3 ,6)));
        assert_eq!(bm.get_coordinate(33), None);
    }
    
    #[test]
    fn get_index_returns_expected_values() {
        let bm = BoardManager::new(3);

        assert_eq!(bm.get_index(&(2, 0)), Some(0));
        assert_eq!(bm.get_index(&(3, 0)), Some(1));
        assert_eq!(bm.get_index(&(3, 3)), Some(16));
        assert_eq!(bm.get_index(&(3 ,6)), Some(31));
        assert_eq!(bm.get_index(&(7, 7)), None);
    }
    
}
