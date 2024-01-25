// https://leetcode.com/problems/longest-common-subsequence/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        if text2.len() > text1.len() {
            return Self::longest_common_subsequence(text2, text1);
        }

        let text1 = text1.as_bytes(); // j
        let text2 = text2.as_bytes(); // i

        let cols = text1.len() + 1; // j
        let rows = text2.len() + 1; // i

        let mut prev_row = vec![0; cols];
        let mut curr_row = vec![0; cols];
        let mut prev = curr_row[0];

        for i in 1..rows {
            // println!("i: {}",i);
            for j in 1..cols {
                // println!("  j: {}",j);
                if text1[j - 1] == text2[i - 1] {
                    curr_row[j] = prev_row[j - 1] + 1;
                } else {
                    curr_row[j] = max(prev_row[j], prev);
                }
                prev = curr_row[j];
            }
            prev_row = curr_row;
            curr_row = vec![0; cols];
            prev = curr_row[0];
            println!("{:?}", &prev_row);
        }

        // println!("{:?}", &dp);
        prev_row[cols - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_common_subsequence("ezupkr".to_string(), "ubmrapg".to_string()),
            2
        )
    }
}
