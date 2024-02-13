
// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/

struct Solution {}

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for i in words {
            if i.chars().rev().collect::<String>() == i {
                return i;
            }
        }

        "".to_string()
    }
}

