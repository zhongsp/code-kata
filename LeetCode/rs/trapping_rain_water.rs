//! https://leetcode.cn/problems/trapping-rain-water/

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut x: i32 = -1;
        let mut y: i32 = -1;
        let mut ans = 0;

        for i in 0..height.len() {
            if height[i] == 0 {
                continue;
            } else {
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
            ans += Self::trap(height.into_iter().map(|v| if v >= 1 { v -= 1 } else { 0 }));
        } else {
            ans
        }
    }
}
