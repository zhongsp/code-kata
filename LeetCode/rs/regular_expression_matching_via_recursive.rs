//! https://leetcode-cn.com/problems/regular-expression-matching/solution/zheng-ze-biao-da-shi-pi-pei-by-leetcode-solution/
//!
//! # 题目分析
//!
//! 最先想到的递归解法，效率不太行，但比较容易理解。
//! 因为工作中经常会用到正则表达式，所以对识别可能的情况比较清晰。
//! 其次，是用到了 Stack 的数据结构，有性能损耗，但是理解容易，不用记索引。
//!
//! 各种普通、特殊的情况见代码里的测试用例。

struct Solution();

impl Solution {
    pub fn to_stack(s: &str) -> Vec<char> {
        s.chars().rev().collect::<Vec<char>>()
    }
    pub fn to_string(stack: &[char]) -> String {
        stack.iter().rev().collect::<String>()
    }

    pub fn is_match(s: String, p: String) -> bool {
        let mut s_stack = Solution::to_stack(&s);
        let mut p_stack = Solution::to_stack(&p);

        while let Some(pattern) = p_stack.pop() {
            match pattern {
                '*' => panic!("A non-'*' character must exist in front of '*'."),
                '.' => {
                    if p_stack.last() == Some(&'*') {
                        p_stack.pop();

                        if p_stack.is_empty() {
                            // when /x.*/
                            return true;
                        } else {
                            // when /.*xy/
                            let p_remaining = Solution::to_string(&p_stack);

                            // 递归检查 /.*/ 后面的模式是否匹配任意一段剩余的字符串
                            while !s_stack.is_empty() {
                                if Solution::is_match(
                                    Solution::to_string(&s_stack),
                                    p_remaining.clone(),
                                ) {
                                    return true;
                                } else {
                                    s_stack.pop();
                                }
                            }

                            // 剩余字符串可能为空字符串
                            return Solution::is_match(String::new(), p_remaining.clone());
                        }
                    }

                    if s_stack.pop().is_none() {
                        return false;
                    }
                }
                pattern => {
                    if p_stack.last() == Some(&'*') {
                        p_stack.pop();

                        if p_stack.is_empty() {
                            // if /x*/, 应该消耗掉字符串所有连续的 'x'。
                            while s_stack.last() == Some(&pattern) {
                                s_stack.pop();
                            }
                            continue;
                        } else {
                            let p_remaining = Solution::to_string(&p_stack);

                            // if /x*yz/，递归检查 /yz/ 是否匹配排除若干个字符 'x' 后的字符串
                            while Some(&pattern) == s_stack.last() {
                                if Solution::is_match(
                                    Solution::to_string(&s_stack),
                                    p_remaining.clone(),
                                ) {
                                    return true;
                                } else {
                                    s_stack.pop();
                                }
                            }

                            // 检查 /yz/ 是否匹配完全排除字符 'x' 后的字符串
                            return Solution::is_match(
                                Solution::to_string(&s_stack),
                                p_remaining.clone(),
                            );
                        }
                    }

                    if s_stack.pop() != Some(pattern) {
                        return false;
                    }
                }
            }
        }

        // 若正则匹配成功，字符串的所有字符应该被消耗
        s_stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_match_0() {
        assert!(!Solution::is_match(String::from("aa"), String::from("a")));
    }

    #[test]
    fn test_is_match_1() {
        assert!(Solution::is_match(String::from("aa"), String::from("a*")));
    }

    #[test]
    fn test_is_match_2() {
        assert!(Solution::is_match(String::from("aa"), String::from("a*")));
    }

    #[test]
    fn test_is_match_3() {
        assert!(Solution::is_match(String::from("ab"), String::from(".*")));
    }

    #[test]
    fn test_is_match_3_1() {
        assert!(Solution::is_match(String::from("ab"), String::from(".*.*")));
    }

    #[test]
    fn test_is_match_4() {
        assert!(Solution::is_match(
            String::from("aab"),
            String::from("c*a*b")
        ));
    }

    #[test]
    fn test_is_match_5() {
        assert!(!Solution::is_match(
            String::from("mississippi"),
            String::from("mis*is*p*.")
        ));
    }

    #[test]
    fn test_is_match_6() {
        assert!(Solution::is_match(String::from("a"), String::from(".*a")));
    }

    #[test]
    fn test_is_match_7() {
        assert!(!Solution::is_match(String::from("a"), String::from("")));
    }

    #[test]
    fn test_is_match_8() {
        assert!(!Solution::is_match(String::from(""), String::from(".")));
    }

    #[test]
    fn test_is_match_9() {
        assert!(Solution::is_match(String::from("aaa"), String::from("a*a")));
    }

    #[test]
    fn test_is_match_10() {
        assert!(Solution::is_match(
            String::from("aab"),
            String::from(".*.ab")
        ));
    }

    #[test]
    fn test_is_match_11() {
        assert!(Solution::is_match(
            String::from("aaa"),
            String::from("ab*a*c*a")
        ));
    }

    #[test]
    fn test_is_match_12() {
        assert!(Solution::is_match(
            String::from("mississippi"),
            String::from("mis*is*ip*.")
        ));
    }

    #[test]
    fn test_is_match_13() {
        assert!(Solution::is_match(
            String::from("abcd"),
            String::from(".*a*")
        ));
    }
}
