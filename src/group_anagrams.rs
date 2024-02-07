// https://leetcode.com/problems/group-anagrams/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut mask = [0; 26];
        let mut hm: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

        for s in strs {
            let mut tmp = s.as_bytes().to_vec();
            tmp.sort_unstable();
            if let Some(x) = hm.get_mut(&tmp) {
                x.push(s);
            } else {
                hm.insert(tmp, vec![s]);
            }
        }

        hm.into_values().collect()
    }
}
