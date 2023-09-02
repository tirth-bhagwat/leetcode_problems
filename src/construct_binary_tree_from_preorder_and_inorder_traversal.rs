// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

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
    pub fn preorder(&self) -> Vec<i32> {
        if self.left.is_none() && self.right.is_none() {
            return vec![self.val];
        }
        let mut res = vec![];

        res.push(self.val);
        if self.left.is_some() {
            res.append(&mut self.left.as_ref().unwrap().borrow().preorder());
        }
        if self.right.is_some() {
            res.append(&mut self.right.as_ref().unwrap().borrow().preorder());
        }
        return res;
    }

    pub fn inorder(&self) -> Vec<i32> {
        if self.left.is_none() && self.right.is_none() {
            return vec![self.val];
        }
        let mut res = vec![];

        if self.left.is_some() {
            res.append(&mut self.left.as_ref().unwrap().borrow().inorder());
        }
        res.push(self.val);
        if self.right.is_some() {
            res.append(&mut self.right.as_ref().unwrap().borrow().inorder());
        }

        return res;
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        println!("preorder: {:?}, inorder: {:?}", preorder, inorder);
        if preorder.len() == 0 || inorder.len() == 0 {
            return None;
        }

        if preorder.len() != inorder.len() {
            panic!("preorder and inorder must have the same length");
        }

        if preorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        }

        let mut root = TreeNode::new(preorder[0]);
        let root_ind_io = inorder.iter().position(|x| *x == root.val).unwrap();
        println!("root_ind_io: {}", root_ind_io);
        if root_ind_io != 0 {
            println!("building left subtree");
            println!("preorder: {:?}", &preorder[1..(root_ind_io + 1)]);
            println!("inorder: {:?}", &inorder[..(root_ind_io)]);
            root.left = Solution::build_tree(
                preorder[1..(root_ind_io + 1)].to_vec(),
                inorder[..(root_ind_io)].to_vec(),
            )
        }

        if root_ind_io != inorder.len() - 1 {
            println!("building right subtree");
            println!("preorder: {:?}", &preorder[root_ind_io + 1..]);
            println!("inorder: {:?}", &inorder[root_ind_io + 1..]);
            root.right = Solution::build_tree(
                preorder[root_ind_io + 1..].to_vec(),
                inorder[root_ind_io + 1..].to_vec(),
            )
        }

        return Some(Rc::new(RefCell::new(root)));
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test() {
        println!("test 1");
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
                .unwrap()
                .borrow()
                .preorder(),
            vec![3, 9, 20, 15, 7],
            "Test 1"
        );
        println!("test 2");
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1])
                .unwrap()
                .borrow()
                .preorder(),
            vec![-1],
            "Test 2"
        );
        println!("test 3");
        assert_eq!(
            Solution::build_tree(vec![1, 2], vec![2, 1])
                .unwrap()
                .borrow()
                .preorder(),
            vec![1, 2],
            "Test 3"
        );
        println!("test 4");
        assert_eq!(
            Solution::build_tree(vec![1, 2, 3], vec![3, 2, 1])
                .unwrap()
                .borrow()
                .preorder(),
            vec![1, 2, 3],
            "Test 4"
        );
        println!("test 5");
        assert_eq!(
            Solution::build_tree(vec![3, 2, 1, 4], vec![1, 2, 3, 4])
                .unwrap()
                .borrow()
                .preorder(),
            vec![3, 2, 1, 4],
            "Test 5"
        );
        println!("test 6");
        assert_eq!(
            Solution::build_tree(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 10],
                vec![4, 3, 5, 2, 6, 1, 8, 7, 11, 9, 10]
            )
            .unwrap()
            .borrow()
            .preorder(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 10],
            "Test 6"
        );
    }
}
