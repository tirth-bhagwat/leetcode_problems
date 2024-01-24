// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/

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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums_found = [0_u32; 9];
        Self::helper(&root, &mut nums_found)
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, nums_found: &mut [u32; 9]) -> i32 {
        let tmp = root.as_ref().unwrap();
        let node = tmp.borrow_mut();
        nums_found[node.val as usize - 1] = nums_found[node.val as usize - 1] + 1;

        let res = match (&node.left, &node.right) {
            (None, None) => {
                if nums_found.iter().filter(|x| (**x % 2) != 0).count() > 1 {
                    0
                } else {
                    1
                }
            }
            (Some(x), Some(y)) => {
                Self::helper(&node.left, nums_found) + Self::helper(&node.right, nums_found)
            }
            (Some(x), None) => Self::helper(&node.left, nums_found),
            (None, Some(x)) => Self::helper(&node.right, nums_found),
        };

        nums_found[node.val as usize - 1] = nums_found[node.val as usize - 1] - 1;
        return res;
    }
}
