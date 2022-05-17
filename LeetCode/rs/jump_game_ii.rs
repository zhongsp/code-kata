//! https://leetcode.cn/problems/jump-game-ii/

struct Solution();

#[allow(dead_code)]
impl Solution {
    // greedy algorithm
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut ans = 0;
        let mut i = 0;

        loop {
            if nums[i] >= (len - 1 - i) as i32 {
                if i < len - 1 {
                    ans += 1;
                }
                break;
            } else {
                let mut max = None;
                let mut jj = i;
                for j in i..=(i + nums[i] as usize) {
                    if max.is_none() || (j as i32 + nums[j] > max.unwrap()) {
                        max = Some(j as i32 + nums[j]);
                        jj = j;
                    }
                }

                ans += 1;
                i = jj;
            }
        }

        ans
    }

    // recursive
    pub fn jump_recursive(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut c = 0;
        loop {
            if Self::is_conn(&nums, 0, nums.len() - 1, c) {
                break;
            }

            c += 1;
        }

        (c + 1) as i32
    }

    pub fn is_conn(n: &Vec<i32>, x: usize, y: usize, c: usize) -> bool {
        println!("{}, {}, {}", x, y, c);
        if c == 2 {
            return true;
        }
        if c == 0 {
            n[x] >= (y - x) as i32
        } else {
            let mut yy = y;

            loop {
                if y - x >= c + 1 {
                    yy -= 1;

                    if yy - x >= c {
                        if n[yy] >= (y - yy) as i32 {
                            if Self::is_conn(&n, x, yy, c - 1) {
                                return true;
                            }
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn jump0() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn jump1() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn jump2() {
        assert_eq!(Solution::jump(vec![2, 0]), 1);
    }

    #[test]
    fn jump3() {
        assert_eq!(Solution::jump(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 9);
    }

    #[test]
    fn jump4() {
        assert_eq!(Solution::jump(vec![0]), 0);
    }

    #[test]
    fn jump5() {
        assert_eq!(
            Solution::jump(vec![
                5, 8, 1, 8, 9, 8, 7, 1, 7, 5, 8, 6, 5, 4, 7, 3, 9, 9, 0, 6, 6, 3, 4, 8, 0, 5, 8, 9,
                5, 3, 7, 2, 1, 8, 2, 3, 8, 9, 4, 7, 6, 2, 5, 2, 8, 2, 7, 9, 3, 7, 6, 9, 2, 0, 8, 2,
                7, 8, 4, 4, 1, 1, 6, 4, 1, 0, 7, 2, 0, 3, 9, 8, 7, 7, 0, 6, 9, 9, 7, 3, 6, 3, 4, 8,
                6, 4, 3, 3, 2, 7, 8, 5, 8, 6, 0
            ]),
            16
        );
    }
}
