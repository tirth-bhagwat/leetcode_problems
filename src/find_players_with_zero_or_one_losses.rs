// https://leetcode.com/problems/find-players-with-zero-or-one-losses/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut hm: HashMap<i32, u8> = HashMap::new();

        for m in matches {
            if hm.get(&m[0]).is_none() {
                hm.insert(m[0], 0);
            }
            if let Some(l) = hm.get_mut(&m[1]) {
                if *l < 2 {
                    *l += 1;
                }
            } else {
                hm.insert(m[1], 1);
            }
        }

        let mut zero = hm
            .iter()
            .filter_map(|(x)| if *x.1 == 0 { Some(*x.0 as i32) } else { None })
            .collect::<Vec<i32>>();

        let mut one = hm
            .iter()
            .filter_map(|(x)| if *x.1 == 1 { Some(*x.0 as i32) } else { None })
            .collect::<Vec<i32>>();

        zero.sort_unstable();
        one.sort_unstable();

        vec![zero, one]
    }
}
