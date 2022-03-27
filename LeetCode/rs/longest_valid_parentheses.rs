//! https://leetcode-cn.com/problems/longest-valid-parentheses/

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![];
        let mut top = 0;

        for (i, char) in s.chars().enumerate() {
            if char == '(' {
                stack.push(i);
            } else {
                if stack.len() - top > 0 {
                    stack.pop();
                } else {
                    stack.push(i);
                    top = stack.len();
                }
            }
        }

        let mut ans = 0;
        let len = stack.len();

        if len == 0 {
            return s.len() as i32;
        }

        for (i, val) in stack.iter().enumerate() {
            if i == 0 {
                ans = *val;
            }

            if i < len - 1 {
                let x = val;
                let y = stack[i + 1];

                ans = if y - x - 1 > ans { y - x - 1 } else { ans };
            }

            if i == len - 1 {
                let x = val;
                let y = s.len() - 1;

                ans = if y - x > ans { y - x } else { ans };
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses0() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from(")()())")),
            4
        );
    }

    #[test]
    fn test_longest_valid_parentheses1() {
        assert_eq!(Solution::longest_valid_parentheses(String::from("(()")), 2);
    }

    #[test]
    fn test_longest_valid_parentheses2() {
        assert_eq!(Solution::longest_valid_parentheses(String::from("")), 0);
    }

    #[test]
    fn test_longest_valid_parentheses3() {
        assert_eq!(Solution::longest_valid_parentheses(String::from("()")), 2);
    }
}
