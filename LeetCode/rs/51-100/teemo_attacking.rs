//! https://leetcode.cn/problems/teemo-attacking/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut ans = 0;
        let mut x = -1;
        let d = duration;

        for i in 0..time_series.len() {
            let y = time_series[i] + d - 1;
            if time_series[i] <= x {
                ans += y - x;
            } else {
                ans += d;
            }
            x = y;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
        assert_eq!(Solution::find_poisoned_duration(vec![0, 0], 2), 2);
        assert_eq!(Solution::find_poisoned_duration(vec![0, 0, 1, 8], 2), 5);
        assert_eq!(Solution::find_poisoned_duration(vec![0, 0, 1, 8, 8], 2), 5);
        assert_eq!(Solution::find_poisoned_duration(vec![0, 0, 1, 8, 9], 2), 6);
    }
}
