//! https://leetcode.cn/problems/minimum-path-sum/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut f = vec![vec![i32::MAX; n + 1]; m + 1];

        f[1][1] = grid[0][0];

        for mi in 1..=m {
            for ni in 1..=n {
                if mi == 1 && ni == 1 {
                    continue;
                }

                f[mi][ni] = i32::min(f[mi - 1][ni], f[mi][ni - 1]) + grid[mi - 1][ni - 1];
            }
        }

        f[m][n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::min_path_sum(vec![vec![1]]), 1);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3]]), 6);
    }
}
