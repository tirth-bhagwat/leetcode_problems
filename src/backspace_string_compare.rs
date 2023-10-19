// https://leetcode.com/problems/backspace-string-compare/

struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_res = String::new();
        let mut t_res = String::new();

        for i in s.chars() {
            match i {
                '#' => {
                    s_res.pop();
                }
                _ => {
                    s_res.push(i);
                }
            }
        }

        for i in t.chars() {
            match i {
                '#' => {
                    t_res.pop();
                }
                _ => {
                    t_res.push(i);
                }
            }
        }

        s_res == t_res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a#c".to_string(), "b".to_string()),
            false
        );
    }
}
