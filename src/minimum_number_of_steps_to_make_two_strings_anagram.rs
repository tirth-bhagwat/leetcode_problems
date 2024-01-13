// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/

struct Solution {}

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_count = [0_i32; 26];
        let mut t_count = [0_i32; 26];

        for (x, y) in s.as_bytes().iter().zip(t.as_bytes()) {
            s_count[(*x - b'a') as usize] += 1;
            s_count[(*y - b'a') as usize] -= 1;
        }

        s_count
            .iter()
            .fold(0, |agg, x| if *x > 0 { agg + x } else { agg })
    }
}
