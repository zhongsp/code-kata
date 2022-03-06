struct Solution();

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

#[allow(dead_code)]
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut list = &head;
        while let Some(node) = list {
            len += 1;
            list = &node.next;
        }

        let index_to_remove = len - n;

        if index_to_remove == 0 {
            return None;
        }

        let mut head = head;
        let mut prev = &mut head;

        for _ in 1..index_to_remove {
            prev = &mut (prev.as_mut().unwrap().next);
        }

        if index_to_remove == (len - 1) {
            prev.as_mut().unwrap().next = None;
        } else {
            prev.as_mut().unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();
        }

        head
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
}
