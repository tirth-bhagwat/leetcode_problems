// https://leetcode.com/problems/find-the-highest-altitude/

use std::vec;

struct Solution {}

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max_pos = 0;
        let mut alts: Vec<i32> = vec![0];

        for i in gain {
            let alt = alts.last().unwrap() + i;
            alts.push(alt);
            if alt > max_pos {
                max_pos = alt;
            }
        }

        return max_pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::largest_altitude(vec![-5, 1, 5, 0, -7]),
            1,
            "Test 1"
        );
        assert_eq!(
            Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]),
            0,
            "Test 2"
        );

        // edge cases
        assert_eq!(Solution::largest_altitude(vec![-1]), 0, "Test 3");
    }
}
