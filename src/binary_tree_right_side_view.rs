// https://leetcode.com/problems/binary-tree-right-side-view/

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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut lev = 1;
        let mut new_lev = 0;
        let mut count = 0;

        if root.is_none() {
            return vec![];
        }

        let mut queue = vec![root];
        let mut res = vec![];

        while let (Some(x)) = queue.pop() {
            let node = x.as_ref().unwrap().borrow();
            println!("'{}' {} {} {}", node.val, lev, new_lev, count);

            match (&node.left, &node.right) {
                (None, None) => {}

                (Some(a), Some(b)) => {
                    queue.insert(0, node.left.clone());
                    queue.insert(0, node.right.clone());
                    new_lev += 2;
                }

                (Some(a), None) => {
                    queue.insert(0, node.left.clone());
                    new_lev += 1;
                }

                (None, Some(a)) => {
                    queue.insert(0, node.right.clone());
                    new_lev += 1;
                }
                _ => {}
            }

            count += 1;
            if lev == count {
                lev = new_lev;
                new_lev = 0;
                count = 0;
                res.push(node.val);
            }

            println!("  '{}' {} {} {}", node.val, lev, new_lev, count);
            println!("  res: {:?}", res);
            println!("  q: {:?}", queue);
        }

        res
    }
}
