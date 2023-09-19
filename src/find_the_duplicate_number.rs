// https://leetcode.com/problems/find-the-duplicate-number/

struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut found = vec![false; n];

        for i in nums {
            if found[i as usize] {
                return i;
            }
            found[i as usize] = true;
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2, "Test 1");
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3, "Test 2");
    }
}
