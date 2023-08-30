// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/

use std::{collections::HashMap, vec};

struct Solution {}

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        if k == 1 {
            if s.chars()
                .into_iter()
                .filter(|x| match x {
                    'a' | 'e' | 'i' | 'o' | 'u' => true,
                    _ => false,
                })
                .count()
                > 0
            {
                return 1;
            } else {
                return 0;
            }
        }

        let mut count = s
            .get(0..k as usize)
            .unwrap()
            .chars()
            .into_iter()
            .filter(|x| match x {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            })
            .count();

        let mut max_count = count;
        for i in 1..s.len() - k as usize + 1 {
            let next = s.get(i + k as usize - 1..i + k as usize).unwrap();
            if Solution::is_vowel(next) {
                count += 1;
            }

            let prev = s.get(i - 1..i).unwrap();
            if Solution::is_vowel(prev) {
                count -= 1;
            }

            if count > max_count {
                max_count = count;
            }
        }

        return max_count as i32;
    }

    pub fn is_vowel(s: &str) -> bool {
        matches!(s, "a" | "e" | "i" | "o" | "u")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2, "Test 1");
        assert_eq!(
            Solution::max_vowels("abciiidef".to_string(), 3),
            3,
            "Test 2"
        );
        assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2, "Test 3");
        assert_eq!(Solution::max_vowels("novowels".to_string(), 1), 1, "Test 4");
        assert_eq!(
            Solution::max_vowels("ryitgacaabwm".to_string(), 4),
            3,
            "Test 5a"
        );
        assert_eq!(
            Solution::max_vowels("tnfazcwrryitgacaabwm".to_string(), 4),
            3,
            "Test 5"
        )
    }
}
