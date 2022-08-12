//! https://leetcode.cn/problems/plus-one/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut add = 1;
        let mut carry = 0;

        for i in (0..digits.len()).rev() {
            if add == 0 && carry == 0 {
                break;
            }

            let sum = digits[i] + add + carry;
            digits[i] = sum % 10;
            carry = sum / 10;
            add = 0;
        }

        if carry == 1 {
            digits.insert(0, 1);
        }

        digits
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
        assert_eq!(Solution::plus_one(vec![1, 9]), vec![2, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
    }
}
