// https://leetcode.com/problems/sort-array-by-parity/

struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut ev: Vec<i32> = Vec::new();
        let mut od: Vec<i32> = Vec::new();
        for i in nums {
            if i & 1 == 0 {
                ev.push(i);
            } else {
                od.push(i);
            }
        }
        ev.extend(od);
        ev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        // Example 1:
        //
        //     Input: nums = [3,1,2,4]
        // Output: [2,4,3,1]
        // Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
        //
        //     Example 2:
        //
        //     Input: nums = [0]
        // Output: [0]
        //

        assert_eq!(
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
            vec![2, 4, 3, 1]
        );

        assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
    }
}
