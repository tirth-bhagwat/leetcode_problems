// https://leetcode.com/problems/find-polygon-with-the-largest-perimeter/

struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        if nums.len() < 3 {
            return -1;
        }
        nums.sort_unstable();
        let nums:Vec<u64> = nums.iter().map(|x| *x as u64).collect();
        let mut max: u64 = 0;
        let mut x = (nums[0] + nums[1]);

        for i in &nums[2..] {
            if x > *i {
                max = x + *i;
            }
            x += *i;
        }

        if max == 0 {
            -1
        } else {
            max as i64
        }
    }
}
