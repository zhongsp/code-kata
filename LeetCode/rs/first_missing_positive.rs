//! https://leetcode-cn.com/problems/first-missing-positive/
//!
//! 从 1 开始的最长序列；最长为正整数个数（要考虑重复数字）？

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut nums: Vec<i32> = nums
            .into_iter()
            .map(|x| if x <= 0 { (len + 1) as i32 } else { x })
            .collect();

        for i in 0..len {
            let val = nums[i].abs() as usize;
            if 1 <= val && val <= len {
                nums[val - 1] = if nums[val - 1] < 0 {
                    nums[val - 1]
                } else {
                    -1 * nums[val - 1]
                }
            }
        }

        for i in 0..len {
            if nums[i] > 0 {
                return (i + 1) as i32;
            }
        }

        (len + 1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_missing_positive0() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2)
    }

    #[test]
    fn first_missing_positive1() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1)
    }
}
