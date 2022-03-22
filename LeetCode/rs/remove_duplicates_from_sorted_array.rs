//! https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        loop {
            if nums.len() <= i + 1 {
                break;
            }

            let left = nums[i];
            let right = nums[i + 1];

            if left == right {
                nums.remove(i + 1);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates0() {
        let mut x: Vec<i32> = vec![1, 1];
        let actual = Solution::remove_duplicates(&mut x);

        println!("{:?}", actual);
    }
}
