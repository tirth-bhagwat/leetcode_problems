// https://leetcode.com/problems/insert-delete-getrandom-o1/

use rand::random;
use std::collections::HashSet;

struct RandomizedSet {
    data: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.data.contains(&val) {
            false
        } else {
            self.data.insert(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.data.contains(&val) {
            self.data.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        *self
            .data
            .iter()
            .nth(rand::random::<usize>() % self.data.len())
            .unwrap()
    }
}

// /**
// * Your RandomizedSet object will be instantiated and called as such:
// * let obj = RandomizedSet::new();
// * let ret_1: bool = obj.insert(val);
// * let ret_2: bool = obj.remove(val);
// * let ret_3: i32 = obj.get_random();
// */
