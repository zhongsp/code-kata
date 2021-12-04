//! https://leetcode-cn.com/problems/reverse-integer/
//!
//! ## 题目分析
//!
//! 反转一串数字，又要限制各种越界。第一感觉就是操作字符串。
//! 因为编程语言一般都提供了丰富的字符串操作函数。
//!
//! 根据数字的正负，先把负号去掉，方便之后的字符串反转。
//! 反转字符串后，再解析成数字。最后把正负号加上。

struct Solution();

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        x.signum()
            * x.abs()
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_0() {
        assert_eq!(Solution::reverse(64), 46i32);
    }

    #[test]
    fn test_reverse_1() {
        assert_eq!(Solution::reverse(0), 0i32);
    }

    #[test]
    fn test_reverse_2() {
        assert_eq!(Solution::reverse(1), 1i32);
    }

    #[test]
    fn test_reverse_3() {
        assert_eq!(Solution::reverse(-1), -1i32);
    }

    #[test]
    fn test_reverse_4() {
        assert_eq!(Solution::reverse(-12), -21i32);
    }

    #[test]
    fn test_reverse_5() {
        assert_eq!(Solution::reverse(1534236469), 0i32);
    }
}
