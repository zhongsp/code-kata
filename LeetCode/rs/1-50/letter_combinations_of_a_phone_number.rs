//! https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/

use std::collections::HashMap;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        match digits.len() {
            0 => vec![],
            1 => Solution::get_chars_of_num(&digits),
            _ => {
                let first: String = digits[..=0].to_owned();
                let others: String = digits[1..].to_owned();
                let head = Solution::get_chars_of_num(&first);
                let tail = Solution::letter_combinations(others);

                let mut result: Vec<String> = vec![];

                for char in &head {
                    for ch in &tail {
                        result.push(char.to_owned() + ch);
                    }
                }
                result
            }
        }
    }

    fn get_chars_of_num(num: &str) -> Vec<String> {
        let keys = HashMap::from([
            ("2", "abc"),
            ("3", "def"),
            ("4", "ghi"),
            ("5", "jkl"),
            ("6", "mno"),
            ("7", "pqrs"),
            ("8", "tuv"),
            ("9", "wxyz"),
        ]);

        return keys
            .get(num)
            .unwrap()
            .chars()
            .map(|s| s.to_string())
            .collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn letter_combinations0() {
        let x = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            x.into_iter().map(|s| s.to_owned()).collect::<Vec<String>>()
        )
    }

    #[test]
    fn letter_combinations1() {
        let x: Vec<&str> = vec![];
        assert_eq!(
            Solution::letter_combinations(String::from("")),
            x.into_iter().map(|s| s.to_owned()).collect::<Vec<String>>()
        )
    }

    #[test]
    fn letter_combinations2() {
        let x = vec!["g", "h", "i"];
        assert_eq!(
            Solution::letter_combinations(String::from("4")),
            x.into_iter().map(|s| s.to_owned()).collect::<Vec<String>>()
        )
    }
}
