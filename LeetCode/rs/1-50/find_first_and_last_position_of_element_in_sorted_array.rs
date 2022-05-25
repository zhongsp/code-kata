//! https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.len() {
            0 => vec![-1, -1],
            len => Self::search_in(nums, target, 0, len - 1),
        }
    }

    fn search_in(nums: Vec<i32>, target: i32, x: usize, y: usize) -> Vec<i32> {
        if x > y {
            return vec![-1, -1];
        }

        let mid = (x + y) / 2;
        if nums[mid] == target {
            Self::search_from(nums, mid)
        } else if nums[mid] > target && mid > x {
            Self::search_in(nums, target, x, mid - 1)
        } else if nums[mid] < target && mid < y {
            Self::search_in(nums, target, mid + 1, y)
        } else {
            vec![-1, -1]
        }
    }

    fn search_from(nums: Vec<i32>, x: usize) -> Vec<i32> {
        let target = nums[x];

        let mut start = x;
        loop {
            if start >= 1 && nums[start - 1] == target {
                start -= 1;
            } else {
                break;
            }
        }

        let mut end = x;
        loop {
            if end + 1 < nums.len() && nums[end + 1] == target {
                end += 1;
            } else {
                break;
            }
        }

        let start = start as i32;
        let end = end as i32;
        vec![start, end]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_range0() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        )
    }

    #[test]
    fn test_search_range1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        )
    }

    #[test]
    fn test_search_range2() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1])
    }

    #[test]
    fn test_search_range3() {
        assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1])
    }

    #[test]
    fn test_search_range4() {
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0])
    }
}
