// https://leetcode.com/problems/reverse-linked-list-ii/

use crate::data_structures::singly_link_list::ListNode;

struct Solution {}

impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode<i32>>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode<i32>>> {
        let mut lst = Vec::<i32>::new();
        let mut curr = &head;

        while let Some(node) = curr {
            lst.push((&node).as_ref().val);
            curr = &node.as_ref().next;
        }

        let mut y = lst
            .clone()
            .get(left as usize - 1..right as usize)
            .unwrap()
            .to_vec();
        y.reverse();
        lst[left as usize - 1..right as usize].swap_with_slice(&mut y);

        return ListNode::from_vec(lst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_between(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2, 4)
                .unwrap()
                .to_str(),
            "14325"
        );
        assert_eq!(
            Solution::reverse_between(ListNode::from_vec(vec![5]), 1, 1)
                .unwrap()
                .to_str(),
            "5"
        );
    }
}
