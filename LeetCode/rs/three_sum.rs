//! https://leetcode-cn.com/problems/3sum/

use std::cmp;
use std::collections::HashMap;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = vec![];
        let mut table: HashMap<String, bool> = HashMap::new();

        let mut start = 0;
        let mut end = nums.len();
        loop {
            if start >= end {
                break;
            }

            Self::sub_three_sum(&nums, &mut result, &mut table, start, end);

            start += 1;
            end -= 1;
        }

        result
    }

    fn sub_three_sum(
        nums: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        table: &mut HashMap<String, bool>,
        start: usize,
        end: usize,
    ) {
        let first = nums[start];
        let last = nums[end - 1];
        let mut index = start + 1;

        loop {
            if index >= end - 1 {
                break;
            }

            if first + last + nums[index] == 0 {
                let pair = Self::sort(first, last, nums[index]);
                if !table.contains_key(&pair.1) {
                    table.insert(pair.1, true);
                    result.push(pair.0);
                }
            }

            index += 1;
        }
    }

    fn sort(x: i32, y: i32, z: i32) -> (Vec<i32>, String) {
        let min = cmp::min(x, cmp::min(y, z));
        let max = cmp::max(x, cmp::max(y, z));
        (vec![min, 0 - min - max, max], format!("{},{}", min, max))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn three_sum0() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        )
    }
}
