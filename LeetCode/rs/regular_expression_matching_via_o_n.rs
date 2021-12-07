//!

struct Solution();

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // setup stack data structure
        let mut s_stack = s.chars().rev().collect::<Vec<char>>();
        let mut p_stack = p.chars().rev().collect::<Vec<char>>();

        while let Some(pattern) = p_stack.pop() {
            match pattern {
                '*' => panic!("A non-'*' character must exist in front of '*'."),
                '.' => {
                    if let Some(ch) = p_stack.pop() {
                        if ch == '*' {
                            if p_stack.is_empty() {
                                return true;
                            } else {
                                let mut is_match_sub = false;
                                for _ in 0..=s_stack.len() {
                                    let sub_match = Solution::is_match(
                                        s_stack.iter().rev().collect::<String>(),
                                        p_stack.iter().rev().collect::<String>(),
                                    );
                                    if sub_match {
                                        is_match_sub = true;
                                        break;
                                    } else {
                                        s_stack.pop();
                                    }
                                }
                                return is_match_sub;
                            }
                        } else {
                            p_stack.push(ch);
                        }
                    }

                    if s_stack.pop().is_none() {
                        return false;
                    }
                }
                pattern => {
                    if let Some(pattern_next) = p_stack.pop() {
                        if pattern_next == '*' {
                            if p_stack.is_empty() {
                                while let Some(ch) = s_stack.pop() {
                                    if ch == pattern {
                                        continue;
                                    } else {
                                        s_stack.push(ch);
                                        break;
                                    }
                                }
                            } else {
                                let mut is_match_sub = false;
                                while Some(&pattern) == s_stack.last() {
                                    let sub_match = Solution::is_match(
                                        s_stack.iter().rev().collect::<String>(),
                                        p_stack.iter().rev().collect::<String>(),
                                    );
                                    if sub_match {
                                        is_match_sub = true;
                                        break;
                                    } else {
                                        s_stack.pop();
                                    }
                                }
                                return is_match_sub
                                    || Solution::is_match(
                                        s_stack.iter().rev().collect::<String>(),
                                        p_stack.iter().rev().collect::<String>(),
                                    );
                            }

                            continue;
                        } else {
                            p_stack.push(pattern_next);
                        }

                        
                    }

                    if s_stack.pop() != Some(pattern) {
                        return false;
                    }
                }
            }
        }

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
}
