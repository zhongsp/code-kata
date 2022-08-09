//! https://leetcode.cn/problems/valid-number/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut has_dot = false;
        let mut has_e = false;
        let mut has_digit_a = false;
        let mut has_digit_b = false;
        let mut has_digit_c = false;
        let mut index_e = 0;

        for (i, ch) in s.bytes().enumerate() {
            match ch {
                b'+' | b'-' => {
                    if i == 0 || (has_e && i == index_e + 1) {
                        continue;
                    } else {
                        return false;
                    }
                }

                b'.' => {
                    if has_dot {
                        return false;
                    } else {
                        if has_e {
                            return false;
                        } else {
                            has_dot = true
                        }
                    }
                }

                b'e' | b'E' => {
                    if has_e {
                        return false;
                    } else {
                        index_e = i;
                        has_e = true
                    }
                }

                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    if has_e {
                        has_digit_c = true;
                    } else if has_dot {
                        has_digit_b = true;
                    } else {
                        has_digit_a = true;
                    }
                }

                _ => return false,
            }
        }

        if (has_digit_a && has_dot)
            || (has_digit_a && has_dot && has_digit_b)
            || (has_dot && has_digit_b)
        {
            if has_e {
                return has_digit_c;
            } else {
                return true;
            }
        }

        if has_digit_a && !has_dot {
            if has_e {
                return has_digit_c;
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::is_number(String::from("0")), true);
        assert_eq!(Solution::is_number(String::from("+0")), true);

        assert_eq!(Solution::is_number(String::from("0.")), true);
        assert_eq!(Solution::is_number(String::from("0.1")), true);
        assert_eq!(Solution::is_number(String::from(".1")), true);

        assert_eq!(Solution::is_number(String::from("0.e0")), true);
        assert_eq!(Solution::is_number(String::from("0.1E0")), true);
        assert_eq!(Solution::is_number(String::from(".1e0")), true);

        assert_eq!(Solution::is_number(String::from("005047e+6")), true);
    }

    #[test]
    fn t1() {
        assert_eq!(Solution::is_number(String::from("e")), false);
        assert_eq!(Solution::is_number(String::from(".")), false);
        assert_eq!(Solution::is_number(String::from("+")), false);

        assert_eq!(Solution::is_number(String::from("0.e")), false);
        assert_eq!(Solution::is_number(String::from("0.1E")), false);
        assert_eq!(Solution::is_number(String::from(".1e")), false);

        assert_eq!(Solution::is_number(String::from("0.e0.")), false);
        assert_eq!(Solution::is_number(String::from("0.1E0.1")), false);
        assert_eq!(Solution::is_number(String::from(".1e.1")), false);

        assert_eq!(Solution::is_number(String::from(".e1")), false);

        assert_eq!(Solution::is_number(String::from("e0")), false);
    }
}
