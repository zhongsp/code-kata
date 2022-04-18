//! https://leetcode-cn.com/problems/sudoku-solver/

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let init = &board.clone();

        Self::fill_current_cell(board, 0, 0, init);
    }

    fn fill_current_cell(board: &mut Vec<Vec<char>>, x: usize, y: usize, init: &Vec<Vec<char>>) {
        if Self::is_cell_fixed(init, x, y) {
            Self::fill_next_cell(board, x, y, init)
        } else {
            let value = board[x][y];
            // let start = if value == '.' { '1' } else { value };
            for z in '1'..='9' {
                board[x][y] = z;
                if Self::is_valid_sudoku(board) {
                    Self::fill_next_cell(board, x, y, init);
                    return;
                }
            }
            Self::fill_prev_cell(board, x, y, init);
        }
    }

    fn fill_current_cell_for_prev(
        board: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        init: &Vec<Vec<char>>,
    ) {
        if Self::is_cell_fixed(init, x, y) {
            Self::fill_prev_cell(board, x, y, init)
        } else {
            let value = board[x][y];
            let start = if value == '.' { '1' } else { char::from_u32(value as u32 + 1).unwrap() };
            for z in start..='9' {
                board[x][y] = z;
                if Self::is_valid_sudoku(board) {
                    Self::fill_next_cell(board, x, y, init);
                    return;
                }
            }
            Self::fill_prev_cell(board, x, y, init);
        }
    }

    fn fill_prev_cell(board: &mut Vec<Vec<char>>, x: usize, y: usize, init: &Vec<Vec<char>>) {
        if y == 0 {
            if x != 0 {
                Self::fill_current_cell_for_prev(board, x - 1, 0, init);
            }
        } else {
            Self::fill_current_cell_for_prev(board, x, y - 1, init);
        }
    }

    fn fill_next_cell(board: &mut Vec<Vec<char>>, x: usize, y: usize, init: &Vec<Vec<char>>) {
        if y == 8 {
            if x != 8 {
                Self::fill_current_cell(board, x + 1, 0, init);
            }
        } else {
            Self::fill_current_cell(board, x, y + 1, init);
        }
    }

    fn is_cell_fixed(board: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        board[x][y] != '.'
    }

    fn is_valid_sudoku(board: &Vec<Vec<char>>) -> bool {
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
