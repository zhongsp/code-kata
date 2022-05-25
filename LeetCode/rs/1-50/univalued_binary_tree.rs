//! https://leetcode-cn.com/problems/implement-strstr/

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::inner(&root.unwrap())
    }

    pub fn inner(root: &Rc<RefCell<TreeNode>>) -> bool {
        let root_val = root.borrow().val;

        if let Some(l) = &root.borrow().left {
            if l.borrow().val == root_val {
                if !Self::inner(l) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(r) = &root.borrow().right {
            if r.borrow().val == root_val {
                if !Self::inner(r) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
