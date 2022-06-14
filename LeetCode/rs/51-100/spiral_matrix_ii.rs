//! https://leetcode.cn/problems/spiral-matrix-ii/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let total = n * n;
        let mut ans: Vec<Vec<i32>> = (0..n).map(|_| vec![0; n as usize]).collect();

        let mut lx: usize = 0;
        let mut ly: usize = 0;
        let mut dir = 0; // 0 right, 1 bottom, 2 left, 3 up
        let mut m = 1;
        ans[0][0] = m;

        loop {
            if m == total {
                break;
            } else {
                m += 1;
            }

            let mut cx = lx;
            let mut cy = ly;

            if dir == 0 {
                if cy + 1 >= n as usize || ans[cx][cy + 1] != 0 {
                    cx += 1;
                    dir = 1;
                } else {
                    cy += 1;
                }
            } else if dir == 1 {
                if cx + 1 >= n as usize || ans[cx + 1][cy] != 0 {
                    cy -= 1;
                    dir = 2;
                } else {
                    cx += 1;
                }
            } else if dir == 2 {
                if cy < 1 || ans[cx][cy - 1] != 0 {
                    cx -= 1;
                    dir = 3;
                } else {
                    cy -= 1;
                }
            } else if dir == 3 {
                if cx < 1 || ans[cx - 1][cy] != 0 {
                    cy += 1;
                    dir = 0;
                } else {
                    cx -= 1;
                }
            }

            ans[cx][cy] = m;
            lx = cx;
            ly = cy;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::generate_matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }
}
