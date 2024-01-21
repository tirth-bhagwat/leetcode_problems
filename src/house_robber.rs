// https://leetcode.com/problems/house-robber/

use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = [-1; 101];
        Solution::rob_ref(&nums, 0, &mut dp)
    }

    pub fn rob_ref(nums: &[i32], ind: usize, dp: &mut [i32; 101]) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            2 => max(nums[0], nums[1]),
            3 => max(nums[0] + nums[2], nums[1]),
            _ => max(
                nums[0] + {
                    if dp[ind] == -1 {
                        dp[ind] = nums[0] + Solution::rob_ref(&nums[2..], ind + 2, dp);
                    }
                    dp[ind] - nums[0]
                },
                nums[1] + {
                    if dp[ind + 1] == -1 {
                        dp[ind + 1] = nums[1] + Solution::rob_ref(&nums[3..], ind + 3, dp);
                    }
                    dp[ind + 1] - nums[1]
                },
            ),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // assert_eq!(Solution::rob(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1]), 4)
        assert_eq!(Solution::rob(vec![3, 1, 0, 0, 2, 3, 1]), 6)
    }
}
