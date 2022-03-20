//! https://leetcode-cn.com/problems/longest-common-prefix/

use std::cmp;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = strs[0].clone();
        for s in strs {
            let len = cmp::min(result.len(), s.len());
            let mut current = String::new();

            for i in 0..len {
                if result.chars().nth(i) == s.chars().nth(i) {
                    current = result.get(..=i).unwrap().to_string();
                } else {
                    break;
                }
            }

            result = current;

            if result.is_empty() {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix_0() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );
    }

    #[test]
    fn test_longest_common_prefix_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            String::from("")
        );
    }
}
