//! https://leetcode-cn.com/problems/remove-element/

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        loop {
            if i >= nums.len() {
                break;
            }

            if nums[i] == val {
                nums.remove(i);
            } else {
                i += 1
            }
        }

        nums.len() as i32
    }
}
