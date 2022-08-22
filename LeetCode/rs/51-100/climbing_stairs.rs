//! https://leetcode.cn/problems/climbing-stairs/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0; n + 2];
        f[0] = 0;
        f[1] = 1;
        f[2] = 2;

        for i in 3..=n {
            f[i] = f[i - 1] + f[i - 2];
        }

        f[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
