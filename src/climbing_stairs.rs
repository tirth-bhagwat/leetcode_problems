// https://leetcode.com/problems/climbing-stairs/

use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut res = 1_u128;
        let max_twos = if n % 2 == 0 { n / 2 } else { (n - 1) / 2 };
        for i in 1..=max_twos {
            let ones = n - (i * 2);

            res += Self::comb((n - i) as u128, max(i, ones) as u128)
                / (if min(i, ones) != 0 {
                    Self::comb(min(i, ones) as u128, 1)
                } else {
                    1
                });
        }

        res as i32
    }

    pub fn comb(len: u128, num: u128) -> u128 {
        let mut res = 1;
        for i in num + 1..=len {
            res *= i;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
        // assert!(false)
    }
}
