//! https://leetcode-cn.com/problems/generate-parentheses/
//! 
//! 采用递归插入一对括号的解法。示例：
//!           ( ) 
//!          ^ ^ ^
//!         插入的位置

use std::collections::HashSet;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: HashSet<String> = HashSet::new();

        // the question says `n >= 1`
        result.insert(String::from(""));

        for _ in 0..n {
            let mut res: HashSet<String> = HashSet::new();
            result.iter().for_each(|s| {
                res.extend(Solution::add_one_pair_parenthesis(s));
            });
            result = res;
        }

        result.into_iter().collect()
    }

    fn add_one_pair_parenthesis(s: &str) -> HashSet<String> {
        let mut result: HashSet<String> = HashSet::new();

        for i in 0..=s.len() {
            let mut parenthesis_str = s.to_owned();
            parenthesis_str.insert_str(i, "()");
            result.insert(parenthesis_str);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_parenthesis0() {
        assert_eq!(Solution::generate_parenthesis(1), vec![String::from("()")])
    }

    #[test]
    fn generate_parenthesis1() {
        assert_eq!(
            Solution::generate_parenthesis(2),
            vec![String::from("(())"), String::from("()()"),]
        )
    }

    #[test]
    fn generate_parenthesis2() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                String::from("((()))"),
                String::from("()(())"),
                String::from("()()()"),
                String::from("(()())"),
                String::from("(())()"),
            ]
        )
    }
}
