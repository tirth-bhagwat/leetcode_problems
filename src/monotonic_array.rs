// https://leetcode.com/problems/monotonic-array/

struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }
        let nums_len = nums.len();
        let mut i = 0;

        while i < nums_len - 1 {
            if nums[i] < nums[i + 1] {
                while i < nums_len - 1 {
                    if nums[i] > nums[i + 1] {
                        return false;
                    }
                    i += 1;
                }
                return true;
            } else if nums[i] > nums[i + 1] {
                while i < nums_len - 1 {
                    if nums[i] < nums[i + 1] {
                        return false;
                    }
                    i += 1;
                }
                return true;
            } else {
                i += 1;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_monotonic() {
        assert_eq!(Solution::is_monotonic(vec![1, 2, 2, 3]), true, "Test 1");
        assert_eq!(Solution::is_monotonic(vec![6, 5, 4, 4]), true, "Test 2");
        assert_eq!(Solution::is_monotonic(vec![1, 3, 2]), false, "Test 3");
        assert_eq!(Solution::is_monotonic(vec![1, 1, 2, 4, 5]), true, "Test 4");
        assert_eq!(Solution::is_monotonic(vec![1, 1, 1]), true, "Test 5");
    }
}
