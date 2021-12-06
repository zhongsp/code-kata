//! https://leetcode-cn.com/problems/palindrome-number/
//!
//! ## 题目分析
//!
//! 第一反应是用字符串操作，因为比较熟悉。
//! 但题目里不让用字符串，然后就想到了数字反转，之前做过一道整数反转的题：
//! https://leetcode-cn.com/problems/reverse-integer/。
//! 需要注意的点就是反转时不能直接反转，因为可能出现数字超出 `i32::MAX` 范围。
//! 然后想到可以略去一位，后来发现可以只反转一半，利用回文的性质。

struct Solution();

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x;
        let mut y = 0;

        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        while x > y {
            y = 10 * y + x % 10;
            x /= 10
        }

        x == y || y / 10 == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_0() {
        assert!(Solution::is_palindrome(0));
    }

    #[test]
    fn test_reverse_1() {
        assert!(Solution::is_palindrome(1));
    }

    #[test]
    fn test_reverse_2() {
        assert!(Solution::is_palindrome(11));
    }

    #[test]
    fn test_reverse_3() {
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn test_reverse_4() {
        assert!(Solution::is_palindrome(111));
    }

    #[test]
    fn test_reverse_5() {
        assert!(!Solution::is_palindrome(112));
    }

    #[test]
    fn test_reverse_6() {
        assert!(Solution::is_palindrome(1221));
    }

    #[test]
    fn test_reverse_7() {
        assert!(!Solution::is_palindrome(1223));
    }

    #[test]
    fn test_reverse_8() {
        assert!(Solution::is_palindrome(3));
    }

    #[test]
    fn test_reverse_9() {
        assert!(Solution::is_palindrome(313));
    }

    #[test]
    fn test_reverse_10() {
        assert!(!Solution::is_palindrome(i32::MAX));
    }
}
