//! https://leetcode-cn.com/problems/3sum/

use std::collections::HashMap;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = vec![];
        let mut table: HashMap<i32, i32> = HashMap::new();

        let mut start = 0;
        let mut end = nums.len();
        loop {
            if end - start < 2 {
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
        table: &mut HashMap<i32, i32>,
        start: usize,
        end: usize,
    ) {
        // todo
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
