//! https://leetcode-cn.com/problems/reverse-integer/
//!
//! ## 题目分析
//!
//! 第一次提交用的是 Rust 提供的 API。但也可以使用算术运算来处理。
//! 想法就是利用求余和求整，然后累加得到最后的结果。
//!
//! 题目中特意强调了“假设环境不允许存储 64 位整数（有符号或无符号）”，
//! 就要求在算法中任何一处算数运算（加减乘除）都不能越界。

struct Solution();

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut y = 0;

        loop {
            let last_digit = x % 10;

            if last_digit >= 0 && (i32::MAX - last_digit) / 10 >= y
                || last_digit < 0 && (i32::MIN - last_digit) / 10 <= y
            {
                x /= 10;
                y = last_digit + y * 10;
            } else {
                break y = 0;
            }

            if x == 0 {
                break;
            }
        }

        y
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
