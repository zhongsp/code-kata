//! https://leetcode-cn.com/problems/3sum/
//!
//! Given a list of numbers, we want to find out:
//! a + b + c = 0

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        if nums.len() < 3 {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = vec![];

        let mut last_a = None;

        for ia in 0..nums.len() {
            let a = nums[ia];

            if last_a == Some(a) {
                continue;
            } else {
                last_a = Some(a);
            }

            let mut ib = ia + 1;
            let mut ic = nums.len() - 1;

            let mut last_b = None;

            loop {
                if ib >= ic {
                    break;
                }

                let b = nums[ib];

                if last_b == Some(b) {
                    ib += 1;
                    continue;
                } else {
                    last_b = Some(b);
                }

                loop {
                    if ib >= ic {
                        break;
                    }

                    let c = nums[ic];

                    let total = a + b + c;
                    if total == 0 {
                        result.push(vec![a, b, c]);
                        ic -= 1;
                        break;
                    } else if total < 0 {
                        break;
                    } else {
                        ic -= 1;
                    }
                }
            }
        }

        result
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

    #[test]
    fn three_sum1() {
        assert_eq!(Solution::three_sum(vec![]), vec![] as Vec<Vec<i32>>)
    }

    #[test]
    fn three_sum2() {
        assert_eq!(Solution::three_sum(vec![0]), vec![] as Vec<Vec<i32>>)
    }
}
