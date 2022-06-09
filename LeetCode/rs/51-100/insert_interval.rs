//! https://leetcode.cn/problems/insert-interval/

use std::cmp;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![new_interval];
        }

        let mut ans = vec![];

        let mut newx = new_interval[0];
        let mut newy = new_interval[1];

        for i in 0..intervals.len() {
            if newy < intervals[i][0] {
                ans.push(vec![newx, newy]);
                ans.extend(intervals.into_iter().skip(i));
                break;
            } else if newx > intervals[i][1] {
                ans.push(intervals[i].to_owned());
                if i == intervals.len() - 1 {
                    ans.push(vec![newx, newy]);
                }
            } else {
                newx = cmp::min(newx, intervals[i][0]);
                newy = cmp::max(newy, intervals[i][1]);
                if i == intervals.len() - 1 {
                    ans.push(vec![newx, newy]);
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
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![2, 3]),
            vec![vec![1, 5]]
        );
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![2, 7]),
            vec![vec![1, 7]]
        );
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![6, 8]),
            vec![vec![1, 5], vec![6, 8]]
        );
    }
}
