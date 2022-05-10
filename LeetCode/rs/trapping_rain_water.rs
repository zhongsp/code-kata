//! https://leetcode.cn/problems/trapping-rain-water/

struct Solution();

#[allow(dead_code)]
impl Solution {
    // O(n)
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut x: i32 = 0;
        let mut ans = 0;
        let mut par = 0;

        for i in 0..height.len() {
            let cur = height[i];

            if cur >= x {
                ans += par;
                par = 0;
                x = cur;
            } else {
                par += x - cur;
            }
        }

        let mut xx: i32 = 0;
        let mut parpar = 0;
        for i in (0..height.len()).rev() {
            let cur = height[i];

            if cur >= xx {
                ans += parpar;
                parpar = 0;
                xx = cur;
            } else {
                parpar += xx - cur;
            }

            if cur == x {
                break;
            }
        }

        ans
    }

    pub fn trap_recursive(height: Vec<i32>) -> i32 {
        let mut x: i32 = -1;
        let mut y: i32 = -1;
        let mut ans = 0;

        for i in 0..height.len() {
            if height[i] == 0 {
                continue;
            } else {
                let i = i as i32;
                if x == -1 {
                    x = i;
                } else if y == -1 {
                    ans += i - x - 1;
                    x = i;
                    y = -1;
                }
            }
        }

        if x >= 0 {
            ans += Self::trap_recursive(
                height
                    .iter()
                    .map(|&v| if v >= 1 { v - 1 } else { 0 })
                    .collect(),
            );
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trap_0() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6)
    }

    #[test]
    fn trap_1() {
        assert_eq!(Solution::trap(vec![0, 1]), 0)
    }
}
