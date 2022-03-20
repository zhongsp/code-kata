struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut last_char = ' ';

        s.chars().for_each(|c| {
            match c {
                'I' => {
                    result += 1;
                }
                'V' => {
                    if last_char == 'I' {
                        result = result - 1 + 4;
                    } else {
                        result += 5;
                    }
                }
                'X' => {
                    if last_char == 'I' {
                        result = result - 1 + 9;
                    } else {
                        result += 10;
                    }
                }
                'L' => {
                    if last_char == 'X' {
                        result = result - 10 + 40
                    } else {
                        result += 50
                    }
                }
                'C' => {
                    if last_char == 'X' {
                        result = result - 10 + 90
                    } else {
                        result += 100
                    }
                }
                'D' => {
                    if last_char == 'C' {
                        result = result - 100 + 400
                    } else {
                        result += 500
                    }
                }
                'M' => {
                    if last_char == 'C' {
                        result = result - 100 + 900
                    } else {
                        result += 1000
                    }
                }
                _ => (),
            };

            last_char = c;
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int_0() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn test_roman_to_int_1() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    }

    #[test]
    fn test_roman_to_int_2() {
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    }

    #[test]
    fn test_roman_to_int_3() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn test_int_to_roman_4() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
