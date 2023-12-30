// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/

struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let words_count = words.len();
        let mut count = [0_usize; 26];

        for w in words {
            for ltr in w.as_bytes() {
                count[(*ltr - b'a') as usize] += 1;
            }
        }

        for c in count {
            if c % words_count != 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::make_equal(vec![
                String::from("abc"),
                String::from("aabc"),
                String::from("bc")
            ]),
            true,
            "Test 1"
        );

        assert_eq!(
            Solution::make_equal(vec![String::from("ab"), String::from("a")]),
            false,
            "Test 2"
        )
    }
}
