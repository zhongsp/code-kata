//! https://leetcode.cn/problems/wildcard-matching/

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::m(&s[..], &p[..])
    }

    pub fn m(s: &str, p: &str) -> bool {
        let mut is = 0;
        let mut ip = 0;

        loop {
            if ip + 1 > p.len() {
                return is >= s.len();
            }

            match p.bytes().nth(ip).unwrap() {
                ch @ b'a'..=b'z' => {
                    if is < s.len() {
                        if s.bytes().nth(is).unwrap() == ch {
                            ip += 1;
                            is += 1;
                            continue;
                        }
                    }
                    return false;
                }
                b'?' => {
                    if is < s.len() {
                        ip += 1;
                        is += 1;
                        continue;
                    }
                    return false;
                }
                b'*' => {
                    if let Some(next) = p.bytes().nth(ip + 1) {
                        match next {
                            b'*' => return Self::m(&s[is..], &p[ip + 1..]),
                            ch @ b'a'..=b'z' => {
                                for i in is..s.len() {
                                    if s.bytes().nth(i).unwrap() == ch {
                                        if Self::m(&s[i..], &p[ip + 1..]) {
                                            return true;
                                        }
                                    }
                                }
                                return false;
                            }
                            b'?' => {
                                for i in is..s.len() {
                                    if Self::m(&s[i..], &p[ip + 1..]) {
                                        return true;
                                    }
                                }
                                return false;
                            }
                            _ => panic!(),
                        }
                    } else {
                        return true;
                    }
                }
                _ => panic!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_match5() {
        assert_eq!(
            Solution::is_match(String::from("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb"), String::from("**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb")),
            true
        );
    }

    #[test]
    fn is_match4() {
        assert_eq!(
            Solution::is_match(String::from("xacab"), String::from("**")),
            true
        );
    }

    #[test]
    fn is_match3() {
        assert_eq!(
            Solution::is_match(String::from("adceb"), String::from("*a*b")),
            true
        );
    }

    #[test]
    fn is_match2() {
        assert_eq!(
            Solution::is_match(String::from("cb"), String::from("?b")),
            true
        );
    }

    #[test]
    fn is_match1() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("*")),
            true
        );
    }

    #[test]
    fn is_match0() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a")),
            false
        );
    }
}
