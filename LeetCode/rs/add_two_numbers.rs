//!https://leetcode-cn.com/problems/add-two-numbers/

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

    #[inline]
    fn from(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

struct Solution();
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // the fixed head node of the final problem result
        let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));

        // The tail node of the problem result.
        // It will change for each calc step.
        let mut tail: &mut Option<Box<ListNode>> = &mut result;

        // The `node1` and `node2` represents the corresponding digits in
        // the current calc. For example, the 2nd node in `l1` and `l2`.
        // Borrow since only reads values of `l1` and `l2`.
        let mut node1: &Option<Box<ListNode>> = &l1;
        let mut node2: &Option<Box<ListNode>> = &l2;
        let mut has_carry_over = false;

        loop {
            let val1 = match node1.as_ref() {
                Some(node) => {
                    node1 = &node.next;
                    node.val
                }
                None => 0,
            };
            let val2 = match node2.as_ref() {
                Some(node) => {
                    node2 = &node.next;
                    node.val
                }
                None => 0,
            };

            let sum = val1 + val2 + if has_carry_over { 1 } else { 0 };

            tail.as_mut().unwrap().val = sum % 10;
            has_carry_over = sum >= 10;

            // All nodes in `l1` and `l2` are processed.
            // And no unresolved carray over digit.
            // Calculation Completed.
            if node1.is_none() && node2.is_none() && !has_carry_over {
                return result;
            } else {
                tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
                tail = &mut tail.as_mut().unwrap().next;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // input: l1 = [0], l2 = [0]
    // output: = [0]
    #[test]
    fn add_two_numbers_1() {
        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));

        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            Some(Box::new(ListNode::new(0)))
        )
    }

    // input: l1 = [2, 4, 3], l2 = [5, 6, 4]
    // output: = [7, 0, 8]
    #[test]
    fn add_two_numbers_2() {
        let l1 = Some(Box::new(ListNode::from(
            2,
            Some(Box::new(ListNode::from(
                4,
                Some(Box::new(ListNode::from(3, None))),
            ))),
        )));
        let l2 = Some(Box::new(ListNode::from(
            5,
            Some(Box::new(ListNode::from(
                6,
                Some(Box::new(ListNode::from(4, None))),
            ))),
        )));

        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            Some(Box::new(ListNode::from(
                7,
                Some(Box::new(ListNode::from(
                    0,
                    Some(Box::new(ListNode::from(8, None))),
                ))),
            )))
        )
    }

    // input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    // output: = [8,9,9,9,0,0,0,1]
    #[test]
    fn add_two_numbers_3() {
        let l1 = Some(Box::new(ListNode::from(
            9,
            Some(Box::new(ListNode::from(
                9,
                Some(Box::new(ListNode::from(
                    9,
                    Some(Box::new(ListNode::from(
                        9,
                        Some(Box::new(ListNode::from(
                            9,
                            Some(Box::new(ListNode::from(
                                9,
                                Some(Box::new(ListNode::from(9, None))),
                            ))),
                        ))),
                    ))),
                ))),
            ))),
        )));
        let l2 = Some(Box::new(ListNode::from(
            9,
            Some(Box::new(ListNode::from(
                9,
                Some(Box::new(ListNode::from(
                    9,
                    Some(Box::new(ListNode::from(9, None))),
                ))),
            ))),
        )));

        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            Some(Box::new(ListNode::from(
                8,
                Some(Box::new(ListNode::from(
                    9,
                    Some(Box::new(ListNode::from(
                        9,
                        Some(Box::new(ListNode::from(
                            9,
                            Some(Box::new(ListNode::from(
                                0,
                                Some(Box::new(ListNode::from(
                                    0,
                                    Some(Box::new(ListNode::from(
                                        0,
                                        Some(Box::new(ListNode::from(1, None,)))
                                    ))),
                                ))),
                            ))),
                        ))),
                    ))),
                ))),
            )))
        )
    }
}
