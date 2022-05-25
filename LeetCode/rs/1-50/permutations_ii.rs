//! https://leetcode.cn/problems/permutations-ii/

use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.len() {
            1 => vec![nums],
            2 => {
                if nums[0] == nums[1] {
                    vec![vec![nums[0], nums[1]]]
                } else {
                    vec![vec![nums[0], nums[1]], vec![nums[1], nums[0]]]
                }
            }
            _ => {
                let mut ans = vec![];
                let mut nums = nums;
                let mut hm = HashSet::new();
                for _ in 0..nums.len() {
                    let fixed = nums.remove(0);
                    if hm.contains(&fixed) {
                        nums.push(fixed);
                        continue;
                    } else {
                        hm.insert(fixed);

                        let subnums = nums.clone();

                        nums.push(fixed);

                        let sub: Vec<Vec<i32>> = Self::permute_unique(subnums)
                            .into_iter()
                            .map(|mut v| {
                                v.insert(0, fixed);
                                v
                            })
                            .collect();
                        ans.extend(sub);
                    }
                }

                ans
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute0() {
        assert_eq!(Solution::permute_unique(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn permute1() {
        assert_eq!(
            Solution::permute_unique(vec![1, 2]),
            vec![vec![1, 2], vec![2, 1]]
        );
    }

    #[test]
    fn permute2() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }
}
