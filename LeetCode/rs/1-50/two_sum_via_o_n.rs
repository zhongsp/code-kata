//! https://leetcode-cn.com/problems/two-sum/

use std::collections::HashMap;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Since the Problem says the result always has 2 elements,
        // so use array on stack instead of Vec on heap for better perf.
        let mut result: [i32; 2] = [0; 2];

        // `key` is the next number to search
        // `value` is the
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            match map.get(num) {
                Some(&i) => {
                    result[0] = i;
                    result[1] = index as i32;
                }
                None => {
                    map.insert(target - num, index as i32);
                }
            }
        }

        // OK, the `Problem` wants a `Vec`.
        result.to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_sum_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn two_sum_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }

    #[test]
    fn two_sum_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1])
    }
}
