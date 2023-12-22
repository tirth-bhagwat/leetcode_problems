// https://leetcode.com/problems/maximum-score-after-splitting-a-string/


struct Solution {}
use std::iter::zip;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut tmp = 0;
        let mut left: Vec<i32> = s
            .chars()
            .take(s.len() - 1)
            .map(|x| {
                if x == '0' {
                    tmp += 1
                }
                tmp
            })
            .collect();
        tmp = 0;
        let mut right: Vec<i32> = s
            .chars()
            .rev()
            .take(s.len() - 1)
            .map(|x| {
                if x == '1' {
                    tmp += 1
                }
                tmp
            })
            .collect();

        right.reverse();

        let mut max = left
            .iter()
            .zip(right.iter())
            .map(|(x, y)| x + y)
            .reduce(|a, b| if a > b { a } else { b })
            .unwrap();

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_score(String::from("011101")), 5, "Test 1");
        assert_eq!(Solution::max_score(String::from("00111")), 5, "Test 2");
    }
}
