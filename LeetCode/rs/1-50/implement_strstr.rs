//! https://leetcode-cn.com/problems/implement-strstr/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let len1 = haystack.len();
        let len2 = needle.len();

        if len2 == 0 {
            return 0;
        }

        for i in 0..len1 {
            if len1 - i < len2 {
                return -1;
            }

            if haystack[i..(i + len2)] == needle {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_str0() {
        assert_eq!(
            Solution::str_str(String::from("abcd"), String::from("c")),
            2
        );
    }

    #[test]
    fn test_str_str1() {
        assert_eq!(
            Solution::str_str(String::from("abcd"), String::from("d")),
            3
        );
    }

    #[test]
    fn test_str_str2() {
        assert_eq!(
            Solution::str_str(String::from("abcd"), String::from("ab")),
            0
        );
    }

    #[test]
    fn test_str_str3() {
        assert_eq!(
            Solution::str_str(String::from("abcd"), String::from("da")),
            -1
        );
    }
}
