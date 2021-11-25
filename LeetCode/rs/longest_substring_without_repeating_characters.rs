//! https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashMap;

struct Solution();
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut max: usize = 0;
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut db: HashMap<char, usize> = HashMap::new();

        loop {
            if end >= s.len() {
                return if db.len() > max {
                    db.len() as i32
                } else {
                    max as i32
                };
            }

            let key = &s[end];
            if db.contains_key(key) {
                let pre = *db.get(key).unwrap();

                if db.len() > max {
                    max = db.len();
                }
                for i in start..pre {
                    db.remove(&s[i]).unwrap();
                }
                *db.get_mut(key).unwrap() = end;
                start = pre + 1;
            } else {
                db.insert(*key, end);
            }

            end += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn length_of_longest_substring1() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        )
    }

    #[test]
    fn length_of_longest_substring2() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        )
    }

    #[test]
    fn length_of_longest_substring3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        )
    }

    #[test]
    fn length_of_longest_substring4() {
        assert_eq!(Solution::length_of_longest_substring(String::from("")), 0)
    }

    #[test]
    fn length_of_longest_substring5() {
        assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1)
    }

    #[test]
    fn length_of_longest_substring6() {
        assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1)
    }

    #[test]
    fn length_of_longest_substring7() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("aabaab!ab")),
            3
        )
    }
}
