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
        let dummy = Some(Box::new(ListNode { next: head, val: 0 }));

        let fast: &Option<Box<ListNode>> = &dummy;
        let slow: &Option<Box<ListNode>> = &dummy;

        // todo

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
