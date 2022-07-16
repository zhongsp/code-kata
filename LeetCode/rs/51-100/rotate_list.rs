//! https://leetcode.cn/problems/rotate-list/

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

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }

        let mut len = 0;
        let mut tmp = &head;
        while let Some(node) = tmp {
            len += 1;
            tmp = &node.next;
        }

        if len == 0 || k == 0 {
            return head;
        }

        let k = k % len;

        if k == 0 {
            return head;
        }

        let k = len - k;

        let mut head = head;
        let mut ans = &mut head;
        for _ in 0..k {
            ans = &mut ans.as_mut().unwrap().next;
        }

        let mut ans = ans.take();
        let mut tmp = &mut ans;
        while tmp.is_some() && tmp.as_ref().unwrap().next.is_some() {
            tmp = &mut tmp.as_mut().unwrap().next;
        }

        // todo
        tmp.as_mut().unwrap().next = head;

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t0() {
        let head = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert_eq!(
            Solution::rotate_right(head, 1),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 0, next: None })),
            }))
        );
    }

    #[test]
    fn t1() {
        let head = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert_eq!(
            Solution::rotate_right(head, 2),
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            }))
        );
    }
}
