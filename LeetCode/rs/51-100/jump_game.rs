//! https://leetcode.cn/problems/jump-game/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();

        if len == 1 {
            return true;
        }

        let mut idx = len;
        for i in 0..len - 1 {
            if nums[i] >= (len - 1 - i) as i32 {
                idx = i;
                break;
            }
        }

        if idx == len {
            false
        } else {
            // 可以用切片优化内存占用
            Self::can_jump(nums[0..=idx].to_owned())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true)
    }

    #[test]
    fn t1() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false)
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::can_jump(vec![0]), true);
        assert_eq!(Solution::can_jump(vec![1]), true);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::can_jump(vec![0, 1]), false);
        assert_eq!(Solution::can_jump(vec![1, 1]), true);
    }
}
