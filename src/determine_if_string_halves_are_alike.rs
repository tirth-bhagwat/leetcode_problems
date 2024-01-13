// https://leetcode.com/problems/determine-if-string-halves-are-alike/

struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let half  = s.len() /2;
        let (mut before,mut after) = (0,0);
        for (i,val) in s.as_bytes().into_iter().enumerate(){

            if i < half {
                before += if matches!(val, b'a'| b'e'| b'i'| b'o'| b'u'| b'A'| b'E'| b'I'| b'O'| b'U') {
                    1
                }
                else {0}
            }
            else {
                after += if matches!(val, b'a'| b'e'| b'i'| b'o'| b'u'| b'A'| b'E'| b'I'| b'O'| b'U') {
                    1
                }
                else {0}
            }
        }

        before == after

    }
}