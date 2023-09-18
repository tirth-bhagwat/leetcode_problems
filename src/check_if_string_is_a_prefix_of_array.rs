// https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/

struct Solution {}

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut start = 0;
        for i in &words {
            if let Some(x) = s.get(start..start + i.len()) {
                if x != i {
                    return false;
                }
                start += i.len();
            } else {
                return false;
            }
            if start == s.len() {
                return true;
            } else if start > s.len() {
                return false;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "apples".to_string(),
                ],
            ),
            true,
            "Test 1"
        );
        assert_eq!(
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "apples".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "leet".to_string(),
                ],
            ),
            false,
            "Test 2"
        );
        assert_eq!(
            Solution::is_prefix_string(
                "ccccccccc".to_string(),
                vec!["c".to_string(), "cc".to_string(),],
            ),
            false,
            "Test 3"
        );
    }
}
