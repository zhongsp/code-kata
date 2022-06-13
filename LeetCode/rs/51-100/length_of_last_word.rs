//! https://leetcode.cn/problems/length-of-last-word/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut ans = 0;
        for ch in s.bytes().rev() {
            if ch == b' ' {
                if ans != 0 {
                    break;
                }
            } else {
                ans += 1;
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
        assert_eq!(
            Solution::length_of_last_word(String::from("Hello World")),
            5
        );
        assert_eq!(
            Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
            4
        );
        assert_eq!(
            Solution::length_of_last_word(String::from("luffy is still joyboy")),
            6
        );
        assert_eq!(Solution::length_of_last_word(String::from(" ")), 0);
        assert_eq!(Solution::length_of_last_word(String::from("a")), 1);
    }
}
