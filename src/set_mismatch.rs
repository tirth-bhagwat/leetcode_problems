// https://leetcode.com/problems/set-mismatch/

use std::ops::Index;

struct Solution {}

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut hm = vec![0_u8; nums.len()];
        for i in nums {
            hm[i as usize - 1] = hm[i as usize - 1] + 1;
        }

        vec![
            (hm.iter().position(|x| *x == 2).unwrap() as i32) + 1,
            (hm.iter().position(|x| *x == 0).unwrap() as i32) + 1,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(Solution::find_error_nums(vec![1, 2, 3, 4, 4]), vec![4, 5]);
        assert_eq!(
            Solution::find_error_nums(vec![3, 2, 3, 4, 6, 5]),
            vec![3, 1]
        );
    }
}
