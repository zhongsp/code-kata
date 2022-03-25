//! https://leetcode-cn.com/problems/divide-two-integers/
//! 
//! 不断翻倍并计数，以得到商；注意各种边界处理。

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }

        let signnum = dividend.signum() * divisor.signum();
        let x = if dividend > 0 { 0 - dividend } else { dividend };
        let mut y = if divisor > 0 { 0 - divisor } else { divisor };

        if y < x {
            return 0;
        }

        let mut ans = 0;

        loop {
            if y < x {
                break;
            }

            if ans == 0 {
                ans = -1;
            } else {
                if ans < i32::MIN - ans {
                    ans = i32::MIN;
                    break;
                } else {
                    ans += ans;
                }
            };

            if y < (i32::MIN - y) {
                break;
            }

            if y + y < x {
                break;
            } else {
                y += y;
            }
        }

        if x - y <= (if divisor > 0 { 0 - divisor } else { divisor }) {
            let n = Self::divide(x - y, divisor);
            let n = n.abs();

            if ans < i32::MIN + n {
                ans = i32::MIN;
            } else {
                ans -= n;
            }
        }

        if ans == i32::MIN && signnum == 1 {
            i32::MAX
        } else {
            -signnum * ans
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_divide0() {
        assert_eq!(Solution::divide(-3, -4), 0);
    }

    #[test]
    fn test_divide1() {
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    }

    #[test]
    fn test_divide2() {
        assert_eq!(Solution::divide(11, 2), 5);
    }

    #[test]
    fn test_divide3() {
        assert_eq!(Solution::divide(2147483647, 1), 2147483647);
    }

    #[test]
    fn test_divide4() {
        assert_eq!(Solution::divide(2147483647, 1), 2147483647);
    }

    #[test]
    fn test_divide5() {
        assert_eq!(Solution::divide(-2147483648, 1), -2147483648);
    }
}
