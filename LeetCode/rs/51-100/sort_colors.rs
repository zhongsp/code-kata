//! https://leetcode.cn/problems/sort-colors/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut cursor_0 = len;
        let mut cursor_1 = len;
        let mut cursor_2 = len;

        for i in (0..len).rev() {
            match nums[i] {
                0 => {
                    if cursor_0 == len {
                        cursor_0 = i;
                    } else {
                        cursor_0 -= 1;
                    }
                }
                1 => {
                    if cursor_1 == len {
                        cursor_1 = cursor_2 - 1;
                        nums[cursor_1] = 1;
                    } else {
                        cursor_1 -= 1;
                        nums[cursor_1] = 1;
                    }
                    if cursor_0 != len {
                        cursor_0 -= 1;
                        nums[cursor_0] = 0;
                    }
                }
                2 => {
                    cursor_2 -= 1;
                    nums[cursor_2] = 2;
                    if cursor_1 != len {
                        cursor_1 -= 1;
                        nums[cursor_1] = 1;
                    }
                    if cursor_0 != len {
                        cursor_0 -= 1;
                        nums[cursor_0] = 0;
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        let mut actual = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn t1() {
        let mut actual = vec![2, 0, 1];
        let expected = vec![0, 1, 2];
        Solution::sort_colors(&mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn t2() {
        let mut actual = vec![2, 0, 2];
        let expected = vec![0, 2, 2];
        Solution::sort_colors(&mut actual);
        assert_eq!(actual, expected);
    }
}
