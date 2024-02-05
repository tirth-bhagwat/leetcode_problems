// https://leetcode.com/problems/first-unique-character-in-a-string/

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts = [0; 26];

        for i in s.bytes() {
            counts[(i - b'a') as usize] += 1;
        }

        for (i, val) in s.bytes().enumerate() {
            if counts[(val - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}
