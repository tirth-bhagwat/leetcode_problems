// https://leetcode.com/problems/find-the-difference/

struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut letters = vec![0_i16; 26];
        let a_pos = 'a' as u8;

        s.chars()
            .into_iter()
            .zip(t.chars().into_iter())
            .for_each(|(x, y)| {
                letters[(x as u8 - a_pos) as usize] += 1;
                letters[(x as u8 - a_pos) as usize] -= 1;
            });
        letters[(t.chars().into_iter().last().unwrap() as u8 - a_pos) as usize] -= 1;

        return (letters.iter().position(|x| (*x == 1 || *x == -1)).unwrap() as u8 + a_pos) as char;
    }
}
