//! https://leetcode.cn/problems/spiral-matrix-ii/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let total = n * n;
        let mut ans: Vec<Vec<i32>> = (0..n).map(|_| Vec::with_capacity(n as usize)).collect();

        let mut lastx = 0;
        let mut lasty = 0;
        let mut dir = 0; // 0 right, 1 bottom, 2 left, 3 up
        let mut m = 1;
        ans[0][0] = m;

        loop {
            if m == total {
                break;
            } else {
                m += 1;
            }

            //calc curx, cury
            let mut cury;
            if dir == 0 {
                if y + 1 >= n {
                    if x + 1 >= n {}
                    dir = 1;
                } else {
                    cury = lasty + 1;
                }
            }
        }

        vec![]
    }
}
