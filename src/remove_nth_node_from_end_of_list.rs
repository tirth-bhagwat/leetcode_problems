// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from_vec(v: Vec<i32>) -> Option<Box<Self>> {
        match v.len() {
            0 => {
                return None;
            }
            1 => {
                return Some(Box::new(ListNode {
                    val: v[0],
                    next: None,
                }));
            }
            _ => {
                return Some(Box::new(ListNode {
                    val: v[0],
                    next: ListNode::from_vec(v[1..].to_vec()),
                }));
            }
        }
    }

    fn to_str(&self) -> String {
        if self.next.is_none() {
            return self.val.to_string();
        }

        return format!(
            "{}{}",
            self.val.to_string(),
            self.next.as_ref().unwrap().to_str()
        );
    }
}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut length = 0;

        let mut next: &Option<Box<ListNode>> = &head;
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

        let mut new_head = ListNode::new(head.as_ref().unwrap().val);
        let mut data_node = &head;

        if n == length {
            new_head = ListNode::new(head.as_ref().unwrap().next.as_ref().unwrap().val);
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
            let new = ListNode::new(data_node.as_ref().unwrap().val);

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
            Solution::remove_nth_from_end(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2)
                .unwrap()
                .to_str(),
            "1235"
        );
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(vec![1]), 1),
            None
        );
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(vec![1, 2]), 1)
                .unwrap()
                .to_str(),
            "1"
        );
        assert_eq!(
            Solution::remove_nth_from_end(ListNode::from_vec(vec![1, 2]), 2)
                .unwrap()
                .to_str(),
            "2"
        );
    }
}
