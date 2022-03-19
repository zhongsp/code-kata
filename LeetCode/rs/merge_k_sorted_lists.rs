//! https://leetcode-cn.com/problems/merge-k-sorted-lists/
//!
//! 朴实无华，不 clone 结点，原地移动。

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut result = Some(Box::new(ListNode { val: 0, next: None }));
        let mut tail = &mut result;

        loop {
            let mut index = None;
            let mut min = None;
            for (k, v) in lists.iter().enumerate() {
                if let Some(node) = v {
                    if min.is_none() || node.val < min.unwrap() {
                        min = Some(node.val);
                        index = Some(k);
                    }
                }
            }

            if index.is_none() {
                break;
            }

            let remaining = lists[index.unwrap()].as_mut().unwrap().next.take();
            tail.as_mut().unwrap().next = lists[index.unwrap()].take();
            tail = &mut tail.as_mut().unwrap().next;
            lists[index.unwrap()] = remaining;
        }

        result.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_merge_k_lists0() {
        let actual = Solution::merge_k_lists(vec![
            Some(Box::new(ListNode { val: 2, next: None })),
            Some(Box::new(ListNode { val: 1, next: None })),
            Some(Box::new(ListNode { val: 3, next: None })),
        ]);

        println!("{:?}", actual);
    }

    #[test]
    pub fn test_merge_k_lists1() {
        let actual = Solution::merge_k_lists(vec![]);
        assert_eq!(actual, None);
    }

    #[test]
    pub fn test_merge_k_lists2() {
        let actual = Solution::merge_k_lists(vec![None]);
        assert_eq!(actual, None);
    }

    #[test]
    pub fn test_merge_k_lists3() {
        let actual = Solution::merge_k_lists(vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        ]);

        println!("{:?}", actual);
    }
}
