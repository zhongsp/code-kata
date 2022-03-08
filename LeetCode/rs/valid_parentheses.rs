//!

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let mut s = s;

        for _ in 0..s.len() {
            let char = s.remove(0);

            match char {
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => stack.push(char),
            }
        }

        stack.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_valid0() {
        assert_eq!(Solution::is_valid(String::from("([{}])")), true)
    }

    #[test]
    fn is_valid1() {
        assert_eq!(Solution::is_valid(String::from("([}])")), false)
    }

    #[test]
    fn is_valid2() {
        assert_eq!(Solution::is_valid(String::from(")[}])")), false)
    }

    #[test]
    fn is_valid3() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true)
    }

    #[test]
    fn is_valid4() {
        assert_eq!(Solution::is_valid(String::from("(]")), false)
    }

    #[test]
    fn is_valid5() {
        assert_eq!(Solution::is_valid(String::from("]")), false)
    }
}
