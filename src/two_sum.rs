// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::new();

        for (i, val) in nums.into_iter().enumerate() {
            if let Some(ind) = hm.get(&(target - val)) {
                return vec![*ind as i32, i as i32];
            } else {
                hm.insert(val, i);
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::two_sum(vec![2, 7, 11, 15], 9),
            vec![0, 1],
            "Test 1"
        );
    }
}
