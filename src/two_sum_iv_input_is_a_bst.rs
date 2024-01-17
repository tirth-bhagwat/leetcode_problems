// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/

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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        Self::helper(&root, &k, &mut hs)
    }
    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, k: &i32, hm: &mut HashSet<i32>) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            return if hm.contains(&(k - n.val)) {
                true
            } else {
                hm.insert(n.val);
                return if Self::helper(&n.left, k, hm) {
                    true
                } else {
                    Self::helper(&n.right, k, hm)
                };
            };
        } else {
            false
        }
    }
}
