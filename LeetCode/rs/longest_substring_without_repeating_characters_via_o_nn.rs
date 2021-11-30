//! https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashMap;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // Not sure if other ways to index chars in a String
        let s: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut max: usize = 0;
        let mut start: usize = 0;
        let mut end: usize = 0;

        // HashSet is another choice
        let mut char_index_map: HashMap<char, usize> = HashMap::new();

        loop {
            if end >= s.len() {
                return if char_index_map.len() > max {
                    char_index_map.len() as i32
                } else {
                    max as i32
                };
            }

            // index into string
            let key = &s[end];
            if char_index_map.contains_key(key) {
                let pre = *char_index_map.get(key).unwrap();

                // save the max length
                if char_index_map.len() > max {
                    max = char_index_map.len();
                }

                // cleanup HashMap for the next finding
                for i in start..pre {
                    char_index_map.remove(&s[i]).unwrap();
                }

                // update value in HashMap
                *char_index_map.get_mut(key).unwrap() = end;

                start = pre + 1;
            } else {
                char_index_map.insert(*key, end);
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
        assert_eq!(
            Solution::length_of_longest_substring(String::from("aabaab!ab")),
            3
        )
    }
}
