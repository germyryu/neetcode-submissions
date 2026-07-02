use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        /*
            Create 3 HashMaps
            - row i -> HashSet
            - column i -> HashSet
            - square i -> HashSet
            
            Requirements:
            - Each HashSet must be a digit 1-9 without duplicates
            - Perform the duplicate check at insert time
        */
        let mut row: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut col: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut square: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..board.len() {
            for j in 0..board.len() {
                // Usually you should perform an isDigit check here and if not '.' but we will ignore that for now
                // Check if the current number is already in row
                if board[i][j] == '.' {
                    continue;
                }
                let current_num = board[i][j].to_digit(10).unwrap() as usize;
                let current_square = (i/3) * 3 + (j/3);
                let r = row.entry(i).or_insert_with(HashSet::new).insert(current_num);
                let c = col.entry(j).or_insert_with(HashSet::new).insert(current_num);
                let s = square.entry(current_square).or_insert_with(HashSet::new).insert(current_num);
                if !r || !c || !s {
                    return false;
                }
            }
        }
        return true;
    }
}
