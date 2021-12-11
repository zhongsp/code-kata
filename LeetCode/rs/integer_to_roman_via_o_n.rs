//! https://leetcode-cn.com/problems/integer-to-roman/
//! 
//! ## 题目分析
//! 
//! 与 10 求余可以得到最后一位的数字；
//! 与 10 整除可以取得下一个待处理的数字；
//! 保存当前的 base 数，如 10, 100, 1000。

struct Solution();

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = vec![];
        let mut base = 1;
        let mut num = num;

        while num != 0 {
            let digit = num % 10;

            match digit * base {
                1 => result.insert(0, "I"),
                2 => result.insert(0, "II"),
                3 => result.insert(0, "III"),
                4 => result.insert(0, "IV"),
                5 => result.insert(0, "V"),
                6 => result.insert(0, "VI"),
                7 => result.insert(0, "VII"),
                8 => result.insert(0, "VIII"),
                9 => result.insert(0, "IX"),
                10 => result.insert(0, "X"),
                20 => result.insert(0, "XX"),
                30 => result.insert(0, "XXX"),
                40 => result.insert(0, "XL"),
                50 => result.insert(0, "L"),
                60 => result.insert(0, "LX"),
                70 => result.insert(0, "LXX"),
                80 => result.insert(0, "LXXX"),
                90 => result.insert(0, "XC"),
                100 => result.insert(0, "C"),
                200 => result.insert(0, "CC"),
                300 => result.insert(0, "CCC"),
                400 => result.insert(0, "CD"),
                500 => result.insert(0, "D"),
                600 => result.insert(0, "DC"),
                700 => result.insert(0, "DCC"),
                800 => result.insert(0, "DCCC"),
                900 => result.insert(0, "CM"),
                1000 => result.insert(0, "M"),
                2000 => result.insert(0, "MM"),
                3000 => result.insert(0, "MMM"),
                _ => (),
            }

            num /= 10;
            base *= 10;
        }

        result.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman_0() {
        assert_eq!(Solution::int_to_roman(3), String::from("III"));
    }

    #[test]
    fn test_int_to_roman_1() {
        assert_eq!(Solution::int_to_roman(4), String::from("IV"));
    }

    #[test]
    fn test_int_to_roman_2() {
        assert_eq!(Solution::int_to_roman(9), String::from("IX"));
    }

    #[test]
    fn test_int_to_roman_3() {
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
    }

    #[test]
    fn test_int_to_roman_4() {
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
    }
}
