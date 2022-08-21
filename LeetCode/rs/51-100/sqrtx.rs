//! https://leetcode.cn/problems/sqrtx/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 0;
        let mut right = x;
        let xx: i64 = x as i64;
        loop {
            if left > right {
                break;
            }

            let mid = (left + right) / 2;

            let mut pow: i64 = mid as i64 * mid as i64;
            if pow == xx {
                return mid;
            } else if pow > xx {
                right = mid - 1;
            } else {
                pow = (mid + 1) as i64 * (mid + 1) as i64;
                if xx < pow {
                    return mid;
                }
                left = mid + 1;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(2), 1);
        assert_eq!(Solution::my_sqrt(3), 1);
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(5), 2);
        assert_eq!(Solution::my_sqrt(6), 2);
        assert_eq!(Solution::my_sqrt(7), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
        assert_eq!(Solution::my_sqrt(i32::MAX), 46340);
    }
}
