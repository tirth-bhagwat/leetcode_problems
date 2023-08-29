// https://leetcode.com/problems/longest-common-prefix/

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common = (&strs[0]).clone();

        for s in &strs {
            let c = Solution::get_common(&common, s);
            if c == "" {
                return "".to_string();
            } else {
                common = Solution::get_common(&common, s);
            }
        }

        return common;
    }

    pub fn get_common(s1: &str, s2: &str) -> String {
        for i in (0..s1.len()+1).rev() {
            if s2.starts_with(s1.get(..i).unwrap()) {
                return s1.get(..i).unwrap().to_string();
            }
        }
        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_common() {
        assert_eq!(Solution::get_common("flower", "flow"), "flow");
        assert_eq!(Solution::get_common("flower", "flight"), "fl");
        assert_eq!(Solution::get_common("flower", "dog"), "");
        assert_eq!(Solution::get_common("a", "a"), "a");
    }


    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );

        // edge cases
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "dog".to_string(),
                "dog".to_string()
            ]),
            "dog".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec!["a".to_string()]),
            "a".to_string()
        );
    }
}
