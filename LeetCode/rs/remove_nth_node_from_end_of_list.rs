//! https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/submissions/
//!
//! 使用双指针，指向链表。因为要修改链表节点，所以要使用可变借用 `&mut`。
//! 但 Rust 只允许同时存在一个可变借用，当存在可变借用时，不允许再有其它任何借用。
//! 所以另一个指针（借用）要指向 `clone` 出来的链表，多占用了内存。

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution();

#[allow(dead_code)]
impl Solution {
    // 双指针解法
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { next: head, val: 0 }));
        let dummy_clone = dummy.clone();

        // slow 可变借用原链表，因为稍后要删除节点。
        // slow 操作的是最终想要的输出结果。
        let mut slow = &mut dummy;

        // fast 只起到循环控制作用，借用拷贝出来的链表即可。
        let mut fast = &dummy_clone;

        // 逐步将 fast 移动到链表结尾；
        // 当 slow 与 fast 之间的间隔为 n 时，开始让 slow 随着 fast 一起移动。
        let mut i = n;
        while let Some(node) = fast {
            fast = &node.next;

            if i >= 0 {
                i -= 1;
            } else {
                slow = &mut slow.as_mut().unwrap().next;
            }
        }

        // 修改链表
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }

    // 计算链表长度
    pub fn remove_nth_from_end_calc_length(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut node = &head;
        while node.is_some() {
            len += 1;
            node = &node.as_ref().unwrap().next;
        }

        let index_to_remove = len - n;

        let mut dummy = Some(Box::new(ListNode { next: head, val: 0 }));
        let mut node = &mut dummy;
        for _ in 0..index_to_remove {
            node = &mut node.as_mut().unwrap().next;
        }

        node.as_mut().unwrap().next = node.as_mut().unwrap().next.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn remove_nth_from_end0() {
        let list: Option<Box<ListNode>> = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode { next: None, val: 5 })),
                        val: 4,
                    })),
                    val: 3,
                })),
                val: 2,
            })),
            val: 1,
        }));
        assert_eq!(
            Solution::remove_nth_from_end(list, 2),
            Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode { next: None, val: 5 })),
                        val: 3,
                    })),
                    val: 2,
                })),
                val: 1,
            }))
        );
    }

    #[test]
    fn remove_nth_from_end1() {
        let list: Option<Box<ListNode>> = Some(Box::new(ListNode { next: None, val: 1 }));
        assert_eq!(Solution::remove_nth_from_end(list, 1), None);
    }

    #[test]
    fn remove_nth_from_end2() {
        let list: Option<Box<ListNode>> = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode { next: None, val: 2 })),
            val: 1,
        }));
        assert_eq!(
            Solution::remove_nth_from_end(list, 1),
            Some(Box::new(ListNode { next: None, val: 1 }))
        );
    }
}
