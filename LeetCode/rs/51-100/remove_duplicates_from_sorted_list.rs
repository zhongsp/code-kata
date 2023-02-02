//! https://leetcode.cn/problems/remove-duplicates-from-sorted-list/

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

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode {
            val: 200,
            next: head,
        });
        let mut current = &mut head;

        while let Some(ref next) = current.next {
            if current.val == next.val {
                current.next = current.next.as_mut().unwrap().next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        head.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        }));
        let ans = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        assert_eq!(Solution::delete_duplicates(head), ans);
    }
}
