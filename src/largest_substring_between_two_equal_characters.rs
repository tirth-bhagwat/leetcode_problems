// https://leetcode.com/problems/largest-substring-between-two-equal-characters/

use std::cmp::{max, Reverse};
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }

        // let mut max_len = -1;
        // let mut hm = [-1; 26];
        //
        // for (i, ch) in s.as_bytes().iter().enumerate() {
        //     let ind = (*ch - b'a') as usize;
        //     match hm[ind] {
        //         -1 => {
        //             hm[ind] = i as i32;
        //         }
        //         _ => {
        //             max_len = max(max_len, i as i32 - hm[ind]);
        //         }
        //     }
        // }
        //
        // if max_len == -1 {
        //     return max_len;
        // }
        //
        // max_len - 1

        let mut s = s.as_bytes();
        let mut hm: HashMap<u8, (i32, i32)> = HashMap::new();

        for (i, ch) in s.iter().enumerate() {
            if let Some(x) = hm.get_mut(ch) {
                x.1 = match x.1 {
                    -1 => i as i32,
                    _ => i as i32,
                };
            } else {
                hm.insert(*ch, (i as i32, -1));
            }
        }

        let mut max = hm
            .into_iter()
            .filter_map(|(k, v)| match v.1 {
                -1 => None,
                _ => Some(v.1 - v.0 - 1),
            })
            .collect::<Vec<i32>>();

        if max.len() == 0 {
            return -1;
        }

        max.sort_by_key(|x| Reverse(*x));

        max[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_length_between_equal_characters("aa".to_string()),
            0,
            "Test 1"
        );

        assert_eq!(
            Solution::max_length_between_equal_characters("abca".to_string()),
            2,
            "Test 2"
        );

        assert_eq!(
            Solution::max_length_between_equal_characters("cbzxy".to_string()),
            -1,
            "Test 3"
        );
    }
}
