//! https://leetcode.cn/problems/search-a-2d-matrix/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut mm: usize = m;
        let mut x = 0;
        let mut y = m;
        while x < y {
            let mid = (x + y) / 2;

            if target > matrix[mid][0] {
                if target <= matrix[mid][n - 1] {
                    mm = mid;
                    break;
                } else {
                    x = mid + 1;
                }
            } else if target < matrix[mid][0] {
                y = mid;
            } else {
                return true;
            }
        }
        if mm == m {
            return false;
        }

        x = 0;
        y = n;
        while x < y {
            let mid = (x + y) / 2;
            if matrix[mm][mid] > target {
                y = mid;
            } else if matrix[mm][mid] < target {
                x = mid + 1;
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
        assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7]], 2), false);
        assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7]], 7), true);
        assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7]], 1), true);
        assert_eq!(Solution::search_matrix(vec![vec![1]], 1), true);
        assert_eq!(Solution::search_matrix(vec![vec![1]], 2), false);
    }
}
