// https://leetcode.com/problems/minimum-falling-path-sum/


struct Solution {}

use std::cmp::min;
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let r_max = &matrix[0].len();
        let c_max = &matrix.len();

        for r in (0..r_max - 1).rev() {
            let prev = (&matrix[r + 1]).clone();
            for c in 0..*c_max {
                match c {
                    0 => {
                        matrix[r][c] += min(prev[c + 1], prev[c]);
                    }
                    _ if c == c_max - 1 => {
                        matrix[r][c] += min(prev[c - 1], prev[c]);
                    }
                    _ => {
                        matrix[r][c] += min(min(prev[c - 1], prev[c]), prev[c + 1]);
                    }
                }
            }
        }

        *matrix[0].iter().min().unwrap()
    }
}
