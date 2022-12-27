//! https://leetcode.cn/problems/combinations/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 1 {
            return (1..=n).map(|i| vec![i]).collect();
        }

        let mut ans: Vec<Vec<i32>> = vec![];

        for cur in (1..=n).rev() {
            let sub = Self::combine(cur - 1, k - 1);

            for mut item in sub {
                item.push(cur);
                ans.push(item);
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::combine(2, 1), vec![vec![1], vec![2]]);
    }
}
