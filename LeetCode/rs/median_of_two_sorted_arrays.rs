//! https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
//!
//! 中位数（Median）又称中值，统计学中的专有名词，是按顺序排列的一组数据中居于中间
//! 位置的数，代表一个样本、种群或概率分布中的一个数值，其可将数值集合划分为相等的上
//! 下两部分。
//! 当 N 为奇数时，X 为中间的那个数。
//! 当 N 为偶数时，X 为中间两个数的平均值。

struct Solution();
impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    //
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn find_median_sorted_arrays1() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2)
  }

  #[test]
  fn find_median_sorted_arrays2() {
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
      2.5
    )
  }

  #[test]
  fn find_median_sorted_arrays3() {
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
      2.5
    )
  }

  #[test]
  fn find_median_sorted_arrays4() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1)
  }

  #[test]
  fn find_median_sorted_arrays5() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2)
  }
}
