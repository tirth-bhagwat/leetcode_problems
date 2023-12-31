use std::fmt::Display;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: Display + Clone + Copy> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Display + Clone + Copy> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
    pub fn from_vec(v: Vec<T>) -> Option<Box<Self>> {
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

    pub fn to_vec(&self) -> Vec<T> {
        if self.next.is_none() {
            return vec![self.val];
        }

        return vec![self.val]
            .into_iter()
            .chain(self.next.as_ref().unwrap().to_vec().into_iter())
            .collect();
    }

    pub fn to_str(&self) -> String {
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
