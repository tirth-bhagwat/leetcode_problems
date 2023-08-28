// https://leetcode.com/problems/implement-stack-using-queues/

struct MyStack {
    q1: Vec<i8>,
    q2: Vec<i8>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack {
            q1: Vec::new(),
            q2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.q1.insert(0, x.try_into().unwrap());
    }

    fn pop(&mut self) -> i32 {
        while (&self).q1.len() != 1 {
            self.q2.insert(0, self.q1.pop().unwrap());
        }
        let tmp = self.q1.pop().unwrap();
        self.q1 = self.q2.clone();
        self.q2 = Vec::new();
        return tmp as i32;
    }

    fn top(&mut self) -> i32 {
        let mut tmp = 0;
        while !(&self).q1.is_empty() {
            tmp = self.q1.pop().unwrap();
            self.q2.insert(0, tmp);
        }
        self.q1 = self.q2.clone();
        self.q2 = Vec::new();
        return tmp as i32;
    }

    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}

//  * Your MyStack object will be instantiated and called as such:
//  * let obj = MyStack::new();
//  * obj.push(x);
//  * let ret_2: i32 = obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: bool = obj.empty();
//  */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut s1 = MyStack::new();
        s1.push(1);
        s1.push(2);
        assert_eq!(s1.top(), 2);
        assert_eq!(s1.pop(), 2);
        assert_eq!(s1.empty(), false);

        assert_eq!(s1.top(), 1);
        assert_eq!(s1.pop(), 1);
        assert_eq!(s1.empty(), true);


        let mut s2 = MyStack::new();
        s2.push(9);
        s2.push(2);
        s2.push(3);
        s2.push(4);
        assert_eq!(s2.top(), 4);
        assert_eq!(s2.pop(), 4);
        assert_eq!(s2.empty(), false);


    }
}
