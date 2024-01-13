// https://leetcode.com/problems/move-zeroes/

struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) -> &mut Vec<i32> {
        let mut zeros = 0;

        for i in nums.iter() {
            if i == &(0 as i32) {
                zeros += 1;
            }
        }

        nums.retain(|&x| x != 0);
        nums.append(&mut vec![0; zeros]);

        return nums;
    }
}
