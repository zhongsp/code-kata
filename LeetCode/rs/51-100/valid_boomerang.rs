//! https://leetcode.cn/problems/valid-boomerang/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let (x0, y0) = (points[0][0], points[0][1]);
        let (x1, y1) = (points[1][0], points[1][1]);
        let (x2, y2) = (points[2][0], points[2][1]);

        if (x0, y0) == (x1, y1) || (x0, y0) == (x2, y2) || (x1, y1) == (x2, y2) {
            return false;
        }

        if (y0 - y1) * (x1 - x2) == (x0 - x1) * (y1 - y2) {
            return false;
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(
            Solution::is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]),
            true
        );
        assert_eq!(
            Solution::is_boomerang(vec![vec![0, 0], vec![2, 3], vec![3, 2]]),
            true
        );
        assert_eq!(
            Solution::is_boomerang(vec![vec![2, 3], vec![0, 0], vec![3, 2]]),
            true
        );
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            false
        );
        assert_eq!(
            Solution::is_boomerang(vec![vec![0, 0], vec![0, 0], vec![0, 0]]),
            false
        );
        assert_eq!(
            Solution::is_boomerang(vec![vec![0, 0], vec![0, 0], vec![1, 1]]),
            false
        );
    }
}
