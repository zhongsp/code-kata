//! https://leetcode.cn/problems/search-in-rotated-sorted-array-ii/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            if left == right {
                return nums[left] == target;
            }

            let mid = (left + right) / 2;

            if mid == left || mid == right {
                return nums[left] == target || nums[right] == target;
            }

            if nums[left] == nums[mid] && nums[mid] == nums[right] {
                left += 1;
                if left < right {
                    right -= 1;
                }
                continue;
            }

            if nums[left] <= nums[mid] {
                if nums[left] <= target && target <= nums[mid] {
                    right = mid;
                } else {
                    left = mid;
                }
                continue;
            }

            if nums[mid] <= nums[right] {
                if nums[mid] <= target && target <= nums[right] {
                    left = mid;
                } else {
                    right = mid;
                }
                continue;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    }

    #[test]
    fn t1() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::search(vec![2], 3), false);
        assert_eq!(Solution::search(vec![3], 3), true);
        assert_eq!(Solution::search(vec![3, 3], 3), true);
        assert_eq!(Solution::search(vec![3, 3], 2), false);
        assert_eq!(Solution::search(vec![1,1,1,1,1,1,1,1,1,13,1,1,1,1,1,1,1,1,1,1,1,1], 13), true);
    }
}
