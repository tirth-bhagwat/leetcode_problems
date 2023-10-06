// https://leetcode.com/problems/design-hashmap/

struct Solution {}

struct MyHashMap {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        MyHashMap { data: vec![] }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.data.len() < key as usize + 1 {
            self.data.resize(key as usize + 1, -1);
        }
        self.data[key as usize] = value;
        println!("put: {:?}", self.data);
    }

    fn get(&self, key: i32) -> i32 {
        println!("get: {:?}", self.data);
        if let Some(v) = self.data.get(key as usize) {
            *v
        } else {
            -1
        }
    }

    fn remove(&mut self, key: i32) {
        if self.data.len() > key as usize {
            self.data[key as usize] = -1;
        }
    }
}

// * Your MyHashMap object will be instantiated and called as such:
// * let obj = MyHashMap::new();
// * obj.put(key, value);
// * let ret_2: i32 = obj.get(key);
// * obj.remove(key);
