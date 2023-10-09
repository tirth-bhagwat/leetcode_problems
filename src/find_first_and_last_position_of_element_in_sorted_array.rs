// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        return match nums.binary_search(&target) {
            Ok(x) => {
                let mut left = x;
                let mut right = x;

                while left > 0 && nums[left - 1] == nums[left] {
                    left -= 1;
                }

                while right < nums.len() - 1 && nums[right + 1] == nums[right] {
                    right += 1;
                }
                vec![left as i32, right as i32]
            }
            Err(_) => {
                vec![-1, -1]
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4],
            "Test 1"
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1],
            "Test 2"
        );
        assert_eq!(Solution::search_range(vec![], 1), vec![-1, -1], "Test 3");
        assert_eq!(Solution::search_range(vec![2, 2], 2), vec![0, 1], "Test 4");
    }
}
