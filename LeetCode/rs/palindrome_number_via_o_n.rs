//! https://leetcode-cn.com/problems/palindrome-number/
//!

struct Solution();

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {}
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
}
