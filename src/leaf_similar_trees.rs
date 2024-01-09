// https://leetcode.com/problems/leaf-similar-trees/

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
use std::rc::Rc;
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::get_leaves(&root1) == Self::get_leaves(&root2)
    }

    pub fn get_leaves(root1: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root1 {
            let n1 = node.borrow_mut();
            if n1.left.is_none() && n1.right.is_none() {
                vec![n1.val]
            } else {
                let mut tmp = Self::get_leaves(&n1.left);
                tmp.append(Self::get_leaves(&n1.right).as_mut());
                tmp
            }
        } else {
            vec![]
        }
    }
}
