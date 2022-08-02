//! https://leetcode.cn/problems/minimum-path-sum/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut f = vec![vec![i32::MAX; n]; m];

        f[0][0] = grid[0][0];

        for mi in 0..m {
            for ni in 0..n {
                if mi == 0 && ni == 0 {
                    continue;
                } else if mi == 0 {
                    f[mi][ni] = f[mi][ni - 1];
                } else if ni == 0 {
                    f[mi][ni] = f[mi - 1][ni];
                } else {
                    f[mi][ni] = i32::min(f[mi - 1][ni], f[mi][ni - 1]);
                }

                f[mi][ni] += grid[mi][ni];
            }
        }

        f[m - 1][n - 1]
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
