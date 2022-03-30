struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.len() {
            0 => -1,
            1 => {
                if nums[0] == target {
                    0
                } else {
                    -1
                }
            }
            len => Self::search_in(nums, target, 0, len - 1),
        }
    }

    pub fn search_in(nums: Vec<i32>, target: i32, x: usize, y: usize) -> i32 {
        if x + 1 == y {
            if nums[x] == target {
                return x as i32;
            } else if nums[y] == target {
                return y as i32;
            } else {
                return -1;
            }
        }

        let mid = (y + x) / 2;

        if nums[x] < nums[mid] {
            if nums[x] <= target && target <= nums[mid] {
                Self::search_in(nums, target, x, mid)
            } else {
                Self::search_in(nums, target, mid, y)
            }
        } else {
            if nums[mid] <= target && target <= nums[y] {
                Self::search_in(nums, target, mid, y)
            } else {
                Self::search_in(nums, target, x, mid)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_search0() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_search1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_search2() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn test_search3() {
        assert_eq!(Solution::search(vec![], 0), -1);
    }
}
