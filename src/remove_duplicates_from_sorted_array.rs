// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev = nums[0];
        let mut unique = 1;
        let mut read = 1;
        let mut write = 1;
        let arr_len = nums.len();

        while read < arr_len {
            if nums[read] != prev {
                if read != write {
                    nums[write] = nums[read];
                }
                write += 1;
                unique += 1;
                prev = nums[read];
            }
            read += 1;
        }
        unique
    }
}
