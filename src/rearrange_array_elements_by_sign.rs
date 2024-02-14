// https://leetcode.com/problems/rearrange-array-elements-by-sign/

struct Solution {}

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = 1;
        let mut res: Vec<i32> = vec![0; nums.len()];
        for n in nums {
            if n < 0 {
                res[j] = n;
                j += 2;
            } else {
                res[i] = n;
                i += 2;
            }
        }
        res
    }
}
