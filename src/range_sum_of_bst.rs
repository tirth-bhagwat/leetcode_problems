// https://leetcode.com/problems/range-sum-of-bst/

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

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Self::count(&root, &low, &high)
    }

    pub fn count(root: &Option<Rc<RefCell<TreeNode>>>, low: &i32, high: &i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let n = node.borrow();
                if &n.val < low {
                    Self::count(&n.right, low, high)
                } else if &n.val > high {
                    Self::count(&n.left, low, high)
                } else {
                    n.val + Self::count(&n.right, low, high) + Self::count(&n.left, low, high)
                }
            }
        }
    }
}
