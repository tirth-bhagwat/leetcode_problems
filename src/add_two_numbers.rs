// https://leetcode.com/problems/add-two-numbers/

use crate::data_structures::singly_link_list::ListNode;

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode<i32>>>,
        l2: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        let mut new_head: Option<Box<ListNode<i32>>> = Some(Box::new(ListNode::new(0)));
        let mut h1 = &l1;
        let mut h2 = &l2;
        let mut curr_node = &mut new_head;
        let (mut dgt_count, mut carry) = (0, 0);
        loop {
            match (h1, h2) {
                (Some(n1), Some(n2)) => {
                    dgt_count += 1;
                    if dgt_count == 1 {
                        curr_node.as_mut().unwrap().val =
                            (n1.as_ref().val + n2.as_ref().val + carry) % 10;
                    } else {
                        let new = Some(Box::new(ListNode::new(
                            (n1.as_ref().val + n2.as_ref().val + carry) % 10,
                        )));
                        curr_node.as_mut().unwrap().next = new;
                        curr_node = &mut curr_node.as_mut().unwrap().next;
                    }
                    carry = (n1.as_ref().val + n2.as_ref().val + carry) / 10;
                    // println!("{} {} {}", n1.as_ref().val, n2.as_ref().val, carry);
                    h1 = &n1.next;
                    h2 = &n2.next;
                }
                (None, Some(n2)) => {
                    dgt_count += 1;
                    if dgt_count == 1 {
                        curr_node.as_mut().unwrap().val = (n2.as_ref().val + carry) % 10;
                    } else {
                        let new = Some(Box::new(ListNode::new((n2.as_ref().val + carry) % 10)));
                        curr_node.as_mut().unwrap().next = new;
                        curr_node = &mut curr_node.as_mut().unwrap().next;
                    }
                    carry = (n2.as_ref().val + carry) / 10;
                    // println!("{} {}", n2.as_ref().val, carry);
                    h2 = &n2.next;
                }
                (Some(n1), None) => {
                    dgt_count += 1;
                    if dgt_count == 1 {
                        curr_node.as_mut().unwrap().val = (n1.as_ref().val + carry) % 10;
                    } else {
                        let new = Some(Box::new(ListNode::new((n1.as_ref().val + carry) % 10)));
                        curr_node.as_mut().unwrap().next = new;
                        curr_node = &mut curr_node.as_mut().unwrap().next;
                    }
                    carry = (n1.as_ref().val + carry) / 10;
                    // println!("{} {}", n1.as_ref().val, carry);
                    h1 = &n1.next;
                }
                (None, None) => {
                    if carry > 0 {
                        let new = Some(Box::new(ListNode::new(carry)));
                        curr_node.as_mut().unwrap().next = new;
                    }
                    return new_head;
                }
            }
        }

        return todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(vec![2, 4, 3]),
                ListNode::from_vec(vec![5, 6, 4])
            )
            .unwrap()
            .to_str(),
            "708",
            "Test 1"
        );
        assert_eq!(
            Solution::add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0]))
                .unwrap()
                .to_str(),
            "0",
            "Test 2"
        );
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
                ListNode::from_vec(vec![9, 9, 9, 9])
            )
            .unwrap()
            .to_str(),
            "89990001",
            "Test 3"
        );

        // edge cases

        // assert_eq!(
        //     Solution::add_two_numbers(
        //         ListNode::from_vec(vec![0]),
        //         ListNode::from_vec(vec![1, 2, 3])
        //     )
        //     .unwrap()
        //     .to_str(),
        //     "321",
        //     "Test 4"
        // );

        // assert_eq!(
        //     Solution::add_two_numbers(
        //         ListNode::from_vec(vec![1, 2, 3]),
        //         ListNode::from_vec(vec![0])
        //     )
        //     .unwrap()
        //     .to_str(),
        //     "321",
        //     "Test 5"
        // );
    }
}
