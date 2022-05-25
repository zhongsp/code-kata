//! https://leetcode-cn.com/problems/zigzag-conversion/
//!
//! ## 题目分析
//!
//!                   P   A   H   N
//! PAYPALISHIRING => A P L S I I G
//!                   Y   I   R
//!
//! 题目要求最后按行输出，即 "PAHNAPLSIIGYIR"。具体方法如下：
//!
//! 1. 为每一行创建一个数组，保存该行的所有字符，最后按序连接该数组即为最终要求的字符串。
//!    例如，应为上例创建 3 个数组：
//!    Arr[0] = ["P",      "A",      "H",      "N"];
//!    Arr[1] = ["A", "P", "L", "S", "I", "I", "G"];
//!    Arr[2] = ["Y",      "I",      "R"          ];
//!
//! 2. 从头开始遍历字符串 “PAYPALISHIRING” 的每一个字符，
//!    判断每个字符应该属于哪一行。
//!    Z 字排列很像波形，有上升沿，有下降沿。可以根据前两个字符的上升和下降趋势来
//!    确定当前字符所在的行。
//!    如果是上升趋势，那么当前行应为前面字符行数减一；反之加一；
//!    波峰（第一行）、波谷（第 `num_rows` 行）单独判断下即可。

use std::cmp;

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut rows = vec![];
        for _ in 0..num_rows {
            rows.push(String::new());
        }

        let s = s.chars().collect::<Vec<char>>();
        let mut row_of_last_last_letter = -1;
        let mut row_of_last_letter = 0;
        for letter in s {
            let row_of_current_letter;
            if row_of_last_letter == 1 {
                row_of_current_letter = cmp::min(row_of_last_letter + 1, num_rows);
            } else if row_of_last_letter == num_rows {
                row_of_current_letter = cmp::max(row_of_last_letter - 1, 1);
            } else if row_of_last_letter > row_of_last_last_letter {
                // go down
                row_of_current_letter = row_of_last_letter + 1;
            } else {
                // go up
                row_of_current_letter = row_of_last_letter - 1;
            }

            row_of_last_last_letter = row_of_last_letter;
            row_of_last_letter = row_of_current_letter;
            rows[row_of_current_letter as usize - 1].push(letter);
        }

        let mut result = String::new();
        for str in rows {
            result.push_str(str.as_str());
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zigzag_conversion_0() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
    }

    #[test]
    fn test_zigzag_conversion_1() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 1),
            String::from("PAYPALISHIRING")
        );
    }

    #[test]
    fn test_zigzag_conversion_2() {
        assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"));
    }

    #[test]
    fn test_zigzag_conversion_3() {
        assert_eq!(
            Solution::convert(String::from("ABC"), 3),
            String::from("ABC")
        );
    }

    #[test]
    fn test_zigzag_conversion_4() {
        assert_eq!(
            Solution::convert(String::from("ABC"), 4),
            String::from("ABC")
        );
    }
}
