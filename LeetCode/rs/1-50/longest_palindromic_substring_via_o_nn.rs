//! https://leetcode-cn.com/problems/longest-palindromic-substring/
//!
//! ## 穷举法 O(N^2)
//!
//! 从 `s` 的第一个字符开始遍历每一个字符 `c`。
//!
//! 1. 以 `c` 为正中心字符，适用于 `ABA` 排列的回文串。
//!    同时向左和右查找是否有对称字符。
//!    如果有对称字符，更新最大回文串的起始索引；然后继续向左和右扩展查找。
//!    结束条件：
//!    1. 无对称字符，或者
//!    2. 左或右到达了字符串 `s` 的端点
//! 2. 以 `c` 和后一个字符为中心，适用于 `ABBA` 排列的回文串。
//!    方法同上

struct Solution();

#[allow(dead_code, unused_comparisons)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let len = chars.len();
        let mut start = 0;
        let mut end = 0;

        for i in 0..len {
            // c is in the middle
            let mut left = i;
            let mut right = i;
            while left >= 0 && right < len {
                let left_most_char_of_c = chars[left];
                let right_most_char_of_c = chars[right];
                if left_most_char_of_c == right_most_char_of_c {
                    if right - left > end - start {
                        start = left;
                        end = right;
                    }

                    if left == 0 {
                        break;
                    } else {
                        left -= 1;
                    }
                    right += 1;
                } else {
                    break;
                }
            }

            // c and c_next are in the middle
            let mut left = i;
            let mut right = i;
            if right + 1 < len && chars[left] == chars[right + 1] {
                right += 1;

                // duplicate with L30
                while left >= 0 && right < len {
                    let left_most_char_of_c = chars[left];
                    let right_most_char_of_c = chars[right];
                    if left_most_char_of_c == right_most_char_of_c {
                        if right - left > end - start {
                            start = left;
                            end = right;
                        }
                        if left == 0 {
                            break;
                        } else {
                            left -= 1;
                        }
                        right += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        s.chars()
            .skip(start)
            .take(end - start + 1)
            .collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn longest_palindrome_0() {
        assert_eq!(
            Solution::longest_palindrome(String::from("babad")),
            String::from("bab")
        );
    }

    #[test]
    fn longest_palindrome_1() {
        assert_eq!(
            Solution::longest_palindrome(String::from("cbbd")),
            String::from("bb")
        );
    }

    #[test]
    fn longest_palindrome_2() {
        assert_eq!(
            Solution::longest_palindrome(String::from("a")),
            String::from("a")
        );
    }

    #[test]
    fn longest_palindrome_3() {
        assert_eq!(
            Solution::longest_palindrome(String::from("ac")),
            String::from("a")
        );
    }

    #[test]
    fn longest_palindrome_4() {
        assert_eq!(
            Solution::longest_palindrome(String::from("acbddbcf")),
            String::from("cbddbc")
        );
    }

    #[test]
    fn longest_palindrome_5() {
        assert_eq!(
            Solution::longest_palindrome(String::from("acbddbc")),
            String::from("cbddbc")
        );
    }

    #[test]
    fn longest_palindrome_6() {
        assert_eq!(
            Solution::longest_palindrome(String::from("abcdd")),
            String::from("dd")
        );
    }
}
