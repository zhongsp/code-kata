//! https://leetcode-cn.com/problems/combination-sum-ii/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();

        let mut res = vec![];

        for i in 0..candidates.len() {
            if candidates[i] <= target {
                res.extend(Self::inner(&candidates, target, i));
            } else {
                break;
            }
        }

        res
    }

    fn inner(candidates: &Vec<i32>, target: i32, index: usize) -> Vec<Vec<i32>> {
        let x = candidates[index];

        if x < target {
            let mut res = vec![];
            for i in (index + 1)..candidates.len() {
                res.extend(
                    Self::inner(candidates, target - x, i)
                        .into_iter()
                        .map(|mut c| {
                            c.insert(0, x);
                            c
                        })
                        .collect::<Vec<Vec<i32>>>(),
                )
            }
            res
        } else if x == target {
            vec![vec![x]]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn combination_sum2_0() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6],]
        )
    }
}
