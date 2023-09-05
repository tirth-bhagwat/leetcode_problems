// https://leetcode.com/problems/minimum-moves-to-reach-target-score/

struct Solution {}

impl Solution {
    pub fn min_moves(mut target: i32, mut max_doubles: i32) -> i32 {
        let mut res = 0;

        while target != 1 && max_doubles > 0 {
            if target & 1 == 0 {
                target /= 2;
                max_doubles -= 1;
            } else {
                target -= 1
            }
            res += 1;
        }

        return res + target - 1;
    }
}
