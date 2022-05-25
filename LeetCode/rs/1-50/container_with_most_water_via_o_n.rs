//! https://leetcode-cn.com/problems/container-with-most-water/
//! 
//! ## 题目分析
//! 
//! 看图凭直觉能感觉到如何移动，问题是如何证明。

use std::cmp;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = height.len() - 1;

        let mut max_volumn = 0;

        while start < end {
            let current_volumn = cmp::min(height[start], height[end]) * (end - start) as i32;

            if current_volumn > max_volumn {
                max_volumn = current_volumn;
            }

            if height[start] <= height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }

        max_volumn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_0() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn test_max_area_1() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn test_max_area_2() {
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }

    #[test]
    fn test_max_area_3() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_max_area_4() {
        assert_eq!(
            Solution::max_area(vec![1, 2, 8, 6, 2, 5, 4, 8, 3, 7, 2, 1]),
            49
        );
    }
}
