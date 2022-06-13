//! https://leetcode.cn/problems/height-checker/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let old = heights.to_owned();
        let mut heights = heights;
        heights.sort();

        let mut ans = 0;
        for i in 0..heights.len() {
            if old[i] != heights[i] {
                ans += 1;
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
        assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
        assert_eq!(Solution::height_checker(vec![5, 1, 2, 3, 4]), 5);
        assert_eq!(Solution::height_checker(vec![1, 2, 3, 4, 5]), 0);
    }
}
