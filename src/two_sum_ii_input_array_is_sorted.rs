// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

use core::num;

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            if numbers[i] + numbers[j] == target {
                return vec![(i as i32) + 1, (j as i32) + 1];
            }

            if numbers[i] + numbers[j] > target {
                j -= 1;
                continue;
            }

            i += 1;
        }

        return vec![0, 0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert!(
            Solution::two_sum(vec![2, 7, 11, 15], 9) == vec![1, 2],
            "test 1"
        );
        assert!(Solution::two_sum(vec![2, 3, 4], 6) == vec![1, 3], "test 2");
        assert!(Solution::two_sum(vec![-1, 0], -1) == vec![1, 2], "test 3");
    }
}
