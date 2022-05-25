//! https://leetcode-cn.com/problems/merge-two-sorted-lists/submissions/

struct Solution();

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut list: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: None }));
        let mut list_ref = &mut list;

        while list1.is_some() || list2.is_some() {
            if list1 == None {
                list_ref.as_mut().unwrap().next = list2.take();
                break;
            }

            if list2 == None {
                list_ref.as_mut().unwrap().next = list1.take();
                break;
            }

            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                let tmp = list1.as_mut().unwrap().next.take();
                list_ref.as_mut().unwrap().next = list1;
                list1 = tmp;
            } else {
                let tmp = list2.as_mut().unwrap().next.take();
                list_ref.as_mut().unwrap().next = list2;
                list2 = tmp;
            }

            list_ref = &mut list_ref.as_mut().unwrap().next;
        }

        list.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_two_lists0() {
        let list1 = Some(Box::new(ListNode { val: 0, next: None }));
        let list2 = Some(Box::new(ListNode { val: 1, next: None }));
        let list = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert_eq!(Solution::merge_two_lists(list1, list2), list);
    }

    #[test]
    fn merge_two_lists1() {
        let list1 = Some(Box::new(ListNode { val: 1, next: None }));
        let list2 = Some(Box::new(ListNode { val: 0, next: None }));
        let list = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert_eq!(Solution::merge_two_lists(list1, list2), list);
    }

    #[test]
    fn merge_two_lists2() {
        let list1 = None;
        let list2 = Some(Box::new(ListNode { val: 0, next: None }));
        let list = Some(Box::new(ListNode { val: 0, next: None }));
        assert_eq!(Solution::merge_two_lists(list1, list2), list);
    }

    #[test]
    fn merge_two_lists3() {
        let list1 = Some(Box::new(ListNode { val: 0, next: None }));
        let list2 = Some(Box::new(ListNode { val: 0, next: None }));
        let list = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 0, next: None })),
        }));
        assert_eq!(Solution::merge_two_lists(list1, list2), list);
    }

    #[test]
    fn merge_two_lists4() {
        let list1 = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let list = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::merge_two_lists(list1, list2), list);
    }
}
