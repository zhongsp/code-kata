//! https://leetcode-cn.com/problems/search-insert-position/

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();

        if len == 0 {
            0
        } else {
            Self::search(nums, target, 0, (len - 1) as usize)
        }
    }

    fn search(nums: Vec<i32>, target: i32, x: usize, y: usize) -> i32 {
        if x == y {
            if nums[x] < target {
                return (x + 1) as i32;
            } else {
                return x as i32;
            }
        }

        let mid = (x + y) / 2;

        if nums[mid] == target {
            mid as i32
        } else if nums[mid] < target {
            Self::search(nums, target, if mid + 1 <= y { mid + 1 } else { y }, y)
        } else {
            Self::search(nums, target, x, if mid >= x + 1 { mid - 1 } else { x })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_search_insert0() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_search_insert1() {
        assert_eq!(Solution::search_insert(vec![1], 7), 1);
    }

    #[test]
    fn test_search_insert2() {
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
    }
}
