// https://leetcode.com/problems/same-tree/

struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl TreeNode {
    pub fn cmp_with(&self, oth: &TreeNode) -> bool {
        if self.val != oth.val {
            return false;
        }

        match (&(self.left), &(oth.left)) {
            (Some(a), Some(b)) => {
                if a.as_ref().borrow().cmp_with(&b.as_ref().borrow()) == false {
                    return false;
                }
            }
            (None, None) => {}
            _ => {
                return false;
            }
        }

        match (&(self.right), &(oth.right)) {
            (Some(a), Some(b)) => {
                return a.as_ref().borrow().cmp_with(&b.as_ref().borrow());
            }
            (None, None) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(a), Some(b)) => {
                return a.as_ref().borrow().cmp_with(&b.as_ref().borrow());
            }

            (None, None) => {
                return true;
            }

            _ => {
                return false;
            }
        }
    }
}
