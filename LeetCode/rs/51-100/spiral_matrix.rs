//! https://leetcode.cn/problems/spiral-matrix/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix = matrix;
        let lm = matrix.len();
        let ln = matrix[0].len();
        let mut ans = vec![];
        let mut m = 0;
        let mut n = 0;
        let mut pm = 0;
        let mut pn = 0;

        while m < lm && n < ln && matrix[m][n] != 101 {
            ans.push(matrix[m][n]);
            matrix[m][n] = 101;

            if pm == m {
                if n >= pn {
                    if n + 1 >= ln || matrix[m][n + 1] == 101 {
                        if m + 1 >= lm || matrix[m + 1][n] == 101 {
                            break;
                        } else {
                            pm = m;
                            pn = n;
                            m += 1;
                        }
                    } else {
                        pm = m;
                        pn = n;
                        n += 1;
                    }
                } else {
                    if n < 1 || matrix[m][n - 1] == 101 {
                        if m < 1 || matrix[m - 1][n] == 101 {
                            break;
                        } else {
                            pm = m;
                            pn = n;
                            m -= 1;
                        }
                    } else {
                        pm = m;
                        pn = n;
                        n -= 1;
                    }
                }
            } else {
                if m > pm {
                    if m + 1 >= lm || matrix[m + 1][n] == 101 {
                        if n < 1 || matrix[m][n - 1] == 101 {
                            break;
                        } else {
                            pm = m;
                            pn = n;
                            n -= 1;
                        }
                    } else {
                        pm = m;
                        pn = n;
                        m += 1;
                    }
                } else {
                    if m < 1 || matrix[m - 1][n] == 101 {
                        if n + 1 >= ln || matrix[m][n + 1] == 101 {
                            break;
                        } else {
                            pm = m;
                            pn = n;
                            n += 1;
                        }
                    } else {
                        pm = m;
                        pn = n;
                        m -= 1;
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        )
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        )
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3, 4]]),
            vec![1, 2, 3, 4,]
        )
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::spiral_order(vec![vec![1]]), vec![1]);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::spiral_order(vec![vec![3], vec![2]]), vec![3, 2]);
    }
}
