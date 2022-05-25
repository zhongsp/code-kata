//! https://leetcode-cn.com/problems/valid-sudoku/

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashMap::new(); 9];
        let mut columns = vec![HashMap::new(); 9];
        let mut boxes = vec![
            vec![HashMap::new(); 3],
            vec![HashMap::new(); 3],
            vec![HashMap::new(); 3],
        ];

        for i in 0..9 {
            for j in 0..9 {
                let val = board[i][j];

                if val == '.' {
                    continue;
                }

                if rows[i].contains_key(&val) {
                    return false;
                } else {
                    rows[i].insert(val, 0);
                }

                if columns[j].contains_key(&val) {
                    return false;
                } else {
                    columns[j].insert(val, 0);
                }

                if boxes[i / 3][j / 3].contains_key(&val) {
                    return false;
                } else {
                    boxes[i / 3][j / 3].insert(val, 0);
                }
            }
        }

        true
    }
}
