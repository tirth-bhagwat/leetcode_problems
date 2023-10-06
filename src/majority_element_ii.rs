// https://leetcode.com/problems/majority-element-ii/

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let threshold = nums.len() / 3;

        let mut hm: HashMap<i32, usize> = HashMap::new();

        let mut res = vec![];
        for i in &nums {
            if let Some(x) = hm.get(&i) {
                if x == &threshold {
                    res.push(*i);
                    if res.len() == 2 {
                        return res;
                    }
                }
                hm.insert(*i, x + 1);
            } else {
                hm.insert(*i, 1);
            }
        }

        for (key, val) in hm.iter() {
            if *val > threshold && !res.contains(key) {
                res.push(*key);
            }
        }
        res.sort();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3], "Test 1");
        assert_eq!(
            Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]),
            vec![1, 2],
            "Test 2"
        );
        assert_eq!(Solution::majority_element(vec![1]), vec![1], "Test 3");
        assert_eq!(Solution::majority_element(vec![1, 2]), vec![1, 2], "Test 4");
        assert_eq!(
            Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]),
            vec![1, 2],
            "Test 5"
        );
    }
}
