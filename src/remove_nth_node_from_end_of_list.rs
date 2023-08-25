// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

use crate::data_structures::singly_link_list::Node;

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<Node<i32>>>, n: i32) -> Option<Box<Node<i32>>> {
        let mut length = 0;

        let mut next: &Option<Box<Node<i32>>> = &head;
        if head.as_ref().unwrap().next.is_none() {
            return None;
        }
        loop {
            match &next {
                Some(x) => {
                    next = &x.next;
                    length += 1;
                }
                None => {
                    break;
                }
            }
        }

        let mut new_head = Node::new(head.as_ref().unwrap().val);
        let mut data_node = &head;

        if n == length {
            new_head = Node::new(head.as_ref().unwrap().next.as_ref().unwrap().val);
            data_node = &head.as_ref().unwrap().next;
        }

        let mut curr = &mut new_head;
        for i in 0..length {
            data_node = &data_node.as_ref().unwrap().next;
            if data_node.is_none() {
                break;
            }
            if i == length - n - 1 {
                continue;
            }
            let new = Node::new(data_node.as_ref().unwrap().val);

            curr.next = Some(Box::new(new));

            curr = curr.next.as_mut().unwrap();
        }

        return Some(Box::new(new_head));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_nth_from_end(Node::from_vec(vec![1, 2, 3, 4, 5]), 2)
                .unwrap()
                .to_str(),
            "1235"
        );
        assert_eq!(
            Solution::remove_nth_from_end(Node::from_vec(vec![1]), 1),
            None
        );
        assert_eq!(
            Solution::remove_nth_from_end(Node::from_vec(vec![1, 2]), 1)
                .unwrap()
                .to_str(),
            "1"
        );
        assert_eq!(
            Solution::remove_nth_from_end(Node::from_vec(vec![1, 2]), 2)
                .unwrap()
                .to_str(),
            "2"
        );
    }
}
