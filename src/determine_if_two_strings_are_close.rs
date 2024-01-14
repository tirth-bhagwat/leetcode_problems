// https://leetcode.com/problems/determine-if-two-strings-are-close/

struct Solution {}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut hm1 = [0_u16; 26];
        let mut hm2 = [0_u16; 26];

        for (x, y) in word1.bytes().zip(word2.bytes()) {
            hm1[(x - b'a') as usize] += 1;
            hm2[(y - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if hm1[i] == 0 || hm2[i] == 0 {
                if hm1[i] == 0 && hm2[i] == 0 {
                    continue;
                } else {
                    return false;
                }
            }
        }

        hm1.sort_unstable();
        hm2.sort_unstable();

        hm1 == hm2
    }
}
