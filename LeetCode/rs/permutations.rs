//! https://leetcode.cn/problems/permutations/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.len() {
            1 => vec![nums],
            2 => vec![vec![nums[0], nums[1]], vec![nums[1], nums[0]]],
            _ => {
                let mut ans = vec![];
                let mut nums = nums;
                for _ in 0..nums.len() {
                    let mut subnums = nums.clone();
                    subnums.remove(0);
                    let sub: Vec<Vec<i32>> = Self::permute(subnums)
                        .into_iter()
                        .map(|mut v| {
                            v.insert(0, nums[0]);
                            v
                        })
                        .collect();
                    ans.extend(sub);
                    let val = nums.remove(0);
                    nums.push(val);
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
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn permute1() {
        assert_eq!(Solution::permute(vec![1, 2]), vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn permute2() {
        assert_eq!(Solution::permute(vec![1, 2, 3]), vec![vec![]]);
    }

    #[test]
    fn permute3() {
        assert_eq!(Solution::permute(vec![1, 2, 3, 4, 5, 6]), vec![vec![]]);
    }
}
