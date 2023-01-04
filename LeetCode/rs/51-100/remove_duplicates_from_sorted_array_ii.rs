//! https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut end = 0usize;

        for cur in 1..nums.len() {
            if nums[cur] != nums[end] {
                // replace
                end += 1;
                nums[end] = nums[cur];
            } else {
                if end == 0 {
                    end += 1;
                } else if nums[end] != nums[end - 1] {
                    end += 1;
                    nums[end] = nums[cur];
                } else {
                    // ignore
                }
            }
        }

        (end as i32) + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
    }

    #[test]
    fn t1() {
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]),
            7
        );
    }
}
