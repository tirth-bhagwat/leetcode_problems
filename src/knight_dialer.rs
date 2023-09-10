// https://leetcode.com/problems/knight-dialer/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    const MOD: i64 = 1_000_000_007;
    pub fn knight_dialer(n: i32) -> i32 {
        let can_travel: Vec<Vec<usize>> = vec![
            vec![4, 6],    // 0
            vec![6, 8],    // 1
            vec![7, 9],    // 2
            vec![4, 8],    // 3
            vec![3, 9, 0], // 4
            vec![],        // 5
            vec![1, 7, 0], // 6
            vec![2, 6],    // 7
            vec![1, 3],    // 8
            vec![2, 4],    // 9
        ];

        let mut prev_data: Vec<i64> = vec![1; 10];

        for _ in 2..=n {
            let mut new: Vec<i64> = Vec::new();
            for i in &can_travel {
                new.push(
                    i.iter()
                        .map(|x| prev_data[*x])
                        .fold(0, |acc, oth| (acc + oth) % Solution::MOD),
                )
            }
            prev_data = new;
        }

        return prev_data
            .iter()
            .fold(0, |acc, oth| ((acc as i64 + oth) % Solution::MOD) as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::knight_dialer(1), 10, "Test 1");
        assert_eq!(Solution::knight_dialer(2), 20, "Test 2");
        assert_eq!(Solution::knight_dialer(3), 46, "Test 3");
        // assert_eq!(Solution::knight_dialer(4), 103, "Test 4");
        // assert_eq!(Solution::knight_dialer(5), 230, "Test 5");
        // assert_eq!(Solution::knight_dialer(6), 603, "Test 6");
        assert_eq!(Solution::knight_dialer(3131), 136006598, "Test 4");
    }
}
