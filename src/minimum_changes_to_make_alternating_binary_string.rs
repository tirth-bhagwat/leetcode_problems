// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/

use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count_0 = 0;
        for i in (0..s.len()).step_by(2) {
            if let Some(ch) = s.get(i..i + 2) {
                count_0 += match ch {
                    "00" => 1,
                    "01" => 0,
                    "10" => 2,
                    "11" => 1,
                    _ => unreachable!(),
                }
            } else {
                count_0 += match s.chars().last().unwrap() {
                    '0' => 0,
                    '1' => 1,
                    _ => unreachable!(),
                };
            }
        }

        min(count_0, (s.len() as i32 - count_0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_operations(String::from("0100")), 1, "Test 1");
        assert_eq!(Solution::min_operations(String::from("1111")), 2, "Test 2");
        assert_eq!(Solution::min_operations(String::from("10")), 0, "Test 3");
    }
}
