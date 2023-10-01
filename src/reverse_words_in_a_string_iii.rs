// https://leetcode.com/problems/reverse-words-in-a-string-iii/

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        // s.split(" ")
        //     .into_iter()
        //     .map(|x| -> String { x.chars().rev().collect::<String>() })
        //     .collect::<Vec<String>>()
        //     .join(" ")

        let mut ch = s.chars().collect::<Vec<char>>();

        let mut start = 0;
        let mut end = 0;
        let s_len = s.len();
        for i in 0..=ch.len() {
            if end >= s_len || ch[i] == ' ' {
                end -= 1;
                while start < end {
                    ch.swap(start, end);
                    start += 1;
                    end -= 1;
                }
                start = i + 1;
                end = start;
            } else {
                end += 1;
            }
        }

        ch.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reverse_words("God Ding".to_string()),
            "doG gniD".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::reverse_words("God".to_string()),
            "doG".to_string()
        )
    }
}
