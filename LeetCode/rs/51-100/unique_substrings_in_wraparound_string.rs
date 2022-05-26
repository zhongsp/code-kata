//! https://leetcode-cn.com/problems/unique_substrings_in_wraparound_string/

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut map: HashMap<u8, i32> = HashMap::new();
        let mut len = 0;
        let mut pre = b'a';

        for (i, ch) in p.bytes().enumerate() {
            if i > 0 && (26 + ch - pre) % 26 == 1 {
                len += 1;
            } else {
                len = 1;
            }
            pre = ch;
            map.insert(ch, *map.get(&ch).unwrap_or(&0).max(&len));
        }

        map.values().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string(String::from("a")),
            1
        )
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string(String::from("cac")),
            2
        )
    }

    #[test]
    fn t2() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string(String::from("zab")),
            6
        )
    }

    #[test]
    fn t4() {
        assert_eq!(
            Solution::find_substring_in_wrapround_string(String::from("zip")),
            3
        )
    }
}
