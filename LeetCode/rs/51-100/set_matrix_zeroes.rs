//! https://leetcode.cn/problems/set-matrix-zeroes/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut row0_has_zero = false;
        let mut col0_has_zero = false;
        for row in 0..m {
            for col in 0..n {
                if matrix[row][col] == 0 {
                    if row == 0 {
                        row0_has_zero = true;
                    }
                    if col == 0 {
                        col0_has_zero = true;
                    }
                    matrix[row][0] = 0;
                    matrix[0][col] = 0;
                }
            }
        }

        for row in 1..m {
            for col in 1..n {
                if matrix[row][0] == 0 || matrix[0][col] == 0 {
                    matrix[row][col] = 0;
                }
            }
        }

        if row0_has_zero {
            for col in 0..n {
                matrix[0][col] = 0;
            }
        }
        if col0_has_zero {
            for row in 0..m {
                matrix[row][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        // assert_eq!(Solution::set_zeroes(), 1);
    }
}
