//! https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
//!
//! 中位数（Median）又称中值，统计学中的专有名词，是按顺序排列的一组数据中居于中间
//! 位置的数，代表一个样本、种群或概率分布中的一个数值，其可将数值集合划分为相等的上
//! 下两部分。
//! - 当 N 为奇数时，X 为中间的那个数。
//! - 当 N 为偶数时，X 为中间两个数的平均值。
//!
//!
//! ## 题目分析
//!
//! 设第一个数组为 A，长度为 M；第二个数组为 B，长度为 N。
//! 那么，题目中的求中位数可转化为如下问题（设 C 为 A 和 B 合且排序后的数组）：
//! 1. 先找到 C 中的第 (M + N + 1) / 2 个数（求整除法），设为 Q。例如：
//!    - 如果 C 的长度 (M + N) 为奇数 5，则 Q 为 C 中的第 (5 + 1) / 2 = 3 个数。
//!    - 如果 C 的长度 (M + N) 为偶数 4，则 Q 为 C 中的第 (4 + 1) / 2 = 2 个数。
//! 2. 得到数 Q 后，简单计算即可得到题目最终要求的 A 和 B 的中位数，即：
//!    - 如果 (M + N) 为奇数，则 A 和 B 的中位数为 Q。
//!    - 如果 (M + N) 为偶数，则 A 和 B 的中位数为 Q 和紧邻 Q 的下一个数的平均数。
//!
//! ### 如何从独立的 A 和 B 中找到第 (M + N + 1) / 2 小的数？
//!
//! 可转化为从 A 和 B 的所有数中挑选出最小的 (M + N + 1) / 2 个数，
//! 然后排在最后的那个数即是。
//!
//!   例如：A 和 B 一共有 4 个数，则从 A 和 B 中一共挑出 2 个最小的数。
//!
//! 挑选出来的最小数分别是来自于 A 和 B。
//!
//!   例如，接上例：
//!       A 中拿 0 个，B 中拿 2 个；
//!       A 中拿 1 个，B 中拿 1 个；
//!       A 中拿 2 个，B 中拿 0 个；
//!
//! 已知 A 和 B 是有序的数组。
//! 设：从 A 中拿的最后一个数的索引为 i，从 B 中拿的最后一个数的索引为 j。
//! 那么，需要满足如下关系：
//! 1. (i + 1) + (j + 1) = (M + N + 1) / 2
//!    即：从 A 和 B 中取出的数字总数为 (M + N + 1) / 2 个。
//!    注：由于 M + N 是已知的，所以 i 的值如果给定了，j 的值也固定了。
//! 2. A[i] <= B[j+1]
//!    B[j] <= A[i+1]
//!    即：中位数的概念，中位数左侧的数必须小于或等于中位数右侧的数。
//!
//! 我们可以遍历 A 或 B 中的元素，来查找满足上述 2 个要求的 i 和 j。
//! 如果依次遍历每一个元素，那么算法的时间复杂度为 O(N)。
//! 如果使用二分查找来遍历元素，那么算法的时间复杂度为 O(log(N))。
//! 题目要求的时间复杂度为：O(log (m+n))，必须使用二分查找。
//!
//! 若 A[i] > B[j+1]，则需要减小 i，即左移。
//! 因为如果右移，那么 A 中取的最后一个元素变大了，B 中取的最后一个元素变小了，更不满足条件。
//! 使用二分查找，一次可以排除一半数量的元素。
//!
//! 最后的难点在于编程时需要进行各种边界检查。
//! 例如，A 全部被选中；或 A 全部没选中，等。

use std::cmp;

pub struct Solution();
impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // 使 nums1 为较短的数组，然后对 nums1 进行二分查找可将时间复杂度进一步降低
    // 为 O(log(min(m, n)))。
    if nums1.len() > nums2.len() {
      return Solution::find_median_sorted_arrays(nums2, nums1);
    }

    let m = nums1.len();
    let n = nums2.len();
    let mut left = 0;
    let mut right = m;

    // median1：前一部分的最大值
    // median2：后一部分的最小值
    let mut median1 = 0;
    let mut median2 = 0;

    while left <= right {
      // 前一部分包含 nums1[0 .. i-1] 和 nums2[0 .. j-1]
      // 后一部分包含 nums1[i .. m-1] 和 nums2[j .. n-1]
      let i = (left + right) / 2;
      let j = (m + n + 1) / 2 - i;

      // nums_im1, nums_i, nums_jm1, nums_j 分别表示 nums1[i-1], nums1[i], nums2[j-1], nums2[j]
      let nums_im1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
      let nums_i = if i == m { i32::MAX } else { nums1[i] };
      let nums_jm1 = if j == 0 { i32::MIN } else { nums2[j - 1] };
      let nums_j = if j == n { i32::MAX } else { nums2[j] };

      if nums_im1 <= nums_j {
        median1 = cmp::max(nums_im1, nums_jm1);
        median2 = cmp::min(nums_i, nums_j);
        left = i + 1;
      } else {
        right = i - 1;
      }
    }

    if (m + n) % 2 == 0 {
      (median1 as f64 + median2 as f64) / 2.0
    } else {
      median1 as f64
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn find_median_sorted_arrays1() {
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
      2.0
    )
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
      0.0
    )
  }

  #[test]
  fn find_median_sorted_arrays4() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0)
  }

  #[test]
  fn find_median_sorted_arrays5() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0)
  }
}
