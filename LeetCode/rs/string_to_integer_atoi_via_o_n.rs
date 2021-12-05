//! https://leetcode-cn.com/problems/string-to-integer-atoi/
//!
//! ## 题目分析
//!
//! 解法题目里已经写得很明白了。

struct Solution();

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut x = 0i32;

        // `-1` if negative; `0` if undetermined; `1` if positive
        let mut signum = 0i32;

        for ch in s.chars() {
            match ch {
                ' ' => {
                    if signum == 0 {
                        continue;
                    } else {
                        break;
                    }
                }
                '-' => {
                    if signum == 0 {
                        signum = -1;
                    } else {
                        break;
                    }
                }
                '+' => {
                    if signum == 0 {
                        signum = 1;
                    } else {
                        break;
                    }
                }
                num @ ('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
                    if signum == 0 {
                        signum = 1;
                    }

                    let num = num.to_digit(10).unwrap() as i32;

                    if signum > 0 && x <= (i32::MAX - num) / 10 {
                        x = x * 10 + num;
                    } else if signum < 0 && x >= (i32::MIN + num) / 10 {
                        x = x * 10 - num;
                    } else {
                        x = if signum > 0 { i32::MAX } else { i32::MIN };
                        break;
                    }
                }
                _ => break,
            }
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi_0() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42i32);
    }

    #[test]
    fn test_my_atoi_1() {
        assert_eq!(Solution::my_atoi(String::from("   -42")), -42i32);
    }

    #[test]
    fn test_my_atoi_2() {
        assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193i32);
    }

    #[test]
    fn test_my_atoi_3() {
        assert_eq!(
            Solution::my_atoi(String::from("   +123 with words")),
            123i32
        );
    }

    #[test]
    fn test_my_atoi_4() {
        assert_eq!(
            Solution::my_atoi(String::from("   +12-3 with words")),
            12i32
        );
    }

    #[test]
    fn test_my_atoi_5() {
        assert_eq!(Solution::my_atoi(String::from("2147483647")), i32::MAX);
    }

    #[test]
    fn test_my_atoi_6() {
        assert_eq!(Solution::my_atoi(String::from("2147483648")), i32::MAX);
    }

    #[test]
    fn test_my_atoi_7() {
        assert_eq!(Solution::my_atoi(String::from("-2147483648")), i32::MIN);
    }

    #[test]
    fn test_my_atoi_8() {
        assert_eq!(Solution::my_atoi(String::from("-2147483649")), i32::MIN);
    }
}
