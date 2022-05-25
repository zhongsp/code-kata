//! https://leetcode-cn.com/problems/3sum-closest/
//!
//! Given a list of numbers, we want to find out:
//! a + b + c = 0

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut result = None;

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

                    match result {
                        None => result = Some(total),
                        Some(last_total) => {
                            if (target - total).abs() < (target - last_total).abs() {
                                result = Some(total);
                            }
                            if total < target {
                                break;
                            } else {
                                ic -= 1;
                            }
                        }
                    }
                }
            }
        }

        result.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn three_sum_closest0() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2)
    }

    #[test]
    fn three_sum_closest1() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn three_sum_closest2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 2, 1, -3], 1), 0);
    }
}
