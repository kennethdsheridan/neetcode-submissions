impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut squares: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    continue;
                }
                let val = board[r][c];
                let square_idx = (r / 3) * 3 + c / 3;

                if !rows[r].insert(val)
                    || !cols[c].insert(val)
                    || !squares[square_idx].insert(val)
                {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_soduku() {
        let input = vec!
        [["1","2",".",".","3",".",".",".","."],
 ["4",".",".","5",".",".",".",".","."],
 [".","9","8",".",".",".",".",".","3"],
 ["5",".",".",".","6",".",".",".","4"],
 [".",".",".","8",".","3",".",".","5"],
 ["7",".",".",".","2",".",".",".","6"],
 [".",".",".",".",".",".","2",".","."],
 [".",".",".","4","1","9",".",".","8"],
 [".",".",".",".","8",".",".","7","9"]];
 
    let result = Solution::is_valid_sodoku(input);
    assert_eq!(result, true)

    }
}