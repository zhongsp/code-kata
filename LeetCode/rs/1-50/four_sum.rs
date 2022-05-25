//! https://leetcode-cn.com/problems/4sum/

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        if nums.len() < 4 {
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

            let mut last_bb = None;

            for ib in (ia + 1)..nums.len() {
                let b = nums[ib];

                if last_bb == Some(b) {
                    continue;
                } else {
                    last_bb = Some(b);
                }

                let mut ic = ib + 1;
                let mut id = nums.len() - 1;

                let mut last_c = None;

                loop {
                    if ic >= id {
                        break;
                    }

                    let c = nums[ic];

                    if last_c == Some(c) {
                        ic += 1;
                        continue;
                    } else {
                        last_c = Some(c);
                    }

                    loop {
                        if ic >= id {
                            break;
                        }

                        let d = nums[id];

                        let total = a + b + c + d;
                        if total == target {
                            result.push(vec![a, b, c, d]);
                            id -= 1;
                            break;
                        } else if total < target {
                            break;
                        } else {
                            id -= 1;
                        }
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
    fn four_sum0() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        )
    }

    #[test]
    fn four_sum1() {
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        )
    }
}
