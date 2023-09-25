// https://leetcode.com/problems/find-the-difference/

struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut letters = vec![0; 26];
        let a_pos = 'a' as usize;

        s.chars().into_iter().for_each(|x| {
            letters[x as usize - a_pos] += 1;
        });
        t.chars().into_iter().for_each(|x| {
            letters[x as usize - a_pos] -= 1;
        });
        return (letters.iter().position(|x| (*x == 1 || *x == -1)).unwrap() + a_pos) as u8 as char;
    }
}
