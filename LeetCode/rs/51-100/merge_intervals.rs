//! https://leetcode.cn/problems/merge-intervals/

use std::cmp;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans = vec![];
        let mut x = -1;
        let mut y = -1;

        for i in intervals {
            if x == -1 {
                x = i[0];
                y = i[1];
            } else {
                if x <= i[0] && i[0] <= y {
                    y = cmp::max(y, i[1]);
                } else {
                    ans.push(vec![x, y]);
                    x = i[0];
                    y = i[1];
                }
            }
        }

        ans.push(vec![x, y]);

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::merge(vec![vec![2, 6], vec![8, 10], vec![1, 3], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        )
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        )
    }
}
