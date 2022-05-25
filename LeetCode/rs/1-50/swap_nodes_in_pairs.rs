//! https://leetcode-cn.com/problems/swap-nodes-in-pairs/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution();

#[allow(dead_code)]
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut first = &mut head;

        loop {
            if first.is_none() {
                break;
            }

            if let Some(node) = first {
                if node.next.is_none() {
                    break;
                }
            }

            let tail = first.as_mut().unwrap().next.as_mut().unwrap().next.take();
            let mut y = first.as_mut().unwrap().next.take();
            let mut x = first.take();
            x.as_mut().unwrap().next = tail;
            y.as_mut().unwrap().next = x;
            *first = y;

            first = &mut first.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn swap_pairs0() {
        let actual = Solution::swap_pairs(Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })));
        println!("{:?}", actual);
    }

    #[test]
    fn swap_pairs1() {
        let actual = Solution::swap_pairs(Some(Box::new(ListNode { val: 0, next: None })));
        println!("{:?}", actual);
    }

    #[test]
    fn swap_pairs2() {
        let actual = Solution::swap_pairs(Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        })));
        println!("{:?}", actual);
    }
}
