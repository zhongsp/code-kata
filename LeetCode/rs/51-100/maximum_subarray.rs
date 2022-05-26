//! https://leetcode.cn/problems/maximum-subarray/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut acc = max;

        for i in 1..nums.len() {
            if acc + nums[i] > 0 {
                acc = acc + nums[i];
            } else {
                acc = nums[i];
            }

            if nums[i] > acc {
                acc = nums[i]
            }

            max = i32::max(acc, max);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
        assert_eq!(Solution::max_sub_array(vec![0]), 0);
    }

    #[test]
    fn t1() {
        assert_eq!(Solution::max_sub_array(vec![1, 2]), 3);
        assert_eq!(Solution::max_sub_array(vec![-1, 2]), 2);
        assert_eq!(Solution::max_sub_array(vec![-1, -2]), -1);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::max_sub_array(vec![-1, -2, 6, -2, 3]), 7);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}
