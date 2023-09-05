// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/

struct Solution {}
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut res = 0;

        while num != 0 {
            if num & 1 == 0 {
                num /= 2;
            } else {
                num -= 1
            }
            res += 1;
        }

        return res;
    }
}
