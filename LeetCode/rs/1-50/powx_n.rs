//! https://leetcode.cn/problems/powx-n/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let sign = n.signum();
        let n = n.abs();

        let ans = if n == 0 {
            1f64
        } else if n == 1 {
            x
        } else {
            let half = n / 2;

            if n % 2 == 0 {
                Self::my_pow(x, half) * Self::my_pow(x, half)
            } else {
                Self::my_pow(x, half) * Self::my_pow(x, half) * x
            }
        };

        if sign > 0 {
            ans
        } else {
            let x = 1f64 / ans;
            if f64::is_finite(x) {
                x
            } else {
                0f64
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_pow0() {
        assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
    }

    #[test]
    fn my_pow1() {
        assert_eq!(Solution::my_pow(2.10000, 3), 9.26100);
    }

    #[test]
    fn my_pow2() {
        assert_eq!(Solution::my_pow(2.00000, -2), 0.25);
    }

    #[test]
    fn my_pow3() {
        assert_eq!(Solution::my_pow(2.00000, -2147483648), 0.25);
    }
}
