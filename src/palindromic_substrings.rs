// https://leetcode.com/problems/palindromic-substrings/

struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut count = 0;
        for i in 0..s.len() {
            count += Solution::num_substring_odd(&s, i as i32);
            if i != s.len() - 1 {
                count += Solution::num_substring_even(&s, i as i32);
            }
        }

        return count;
    }

    pub fn num_substring_odd(s: &String, ind: i32) -> i32 {
        let mut left = ind;
        let mut right = ind + 1;
        let mut count = 0;

        while left > -1 && right <= s.len() as i32 {
            if Solution::is_palin(s.get(left as usize..right as usize).unwrap().to_string()) {
                count += 1;
            } else {
                return count;
            }

            left -= 1;
            right += 1;
        }

        return count;
    }

    pub fn num_substring_even(s: &String, ind: i32) -> i32 {
        let mut left = ind;
        let mut right = ind + 2;
        let mut count = 0;

        while left > -1 && right <= s.len() as i32 {
            if Solution::is_palin(s.get(left as usize..right as usize).unwrap().to_string()) {
                count += 1;
            } else {
                return count;
            }

            left -= 1;
            right += 1;
        }

        return count;
    }

    pub fn is_palin(s: String) -> bool {
        if s.len() == 1 {
            return true;
        }
        let half = s.len() / 2;
        if s.len() % 2 == 0 {
            return s.get(0..half).unwrap().eq(&s
                .get(half..)
                .unwrap()
                .chars()
                .rev()
                .collect::<String>());
        } else {
            return s.get(0..half).unwrap().eq(&s
                .get(half + 1..)
                .unwrap()
                .chars()
                .rev()
                .collect::<String>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3, "Test 1");
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6, "Test 2");

        assert_eq!(
            Solution::count_substrings("abccba".to_string()),
            9,
            "Test 3"
        );

        assert_eq!(Solution::count_substrings("a".to_string()), 1, "Test 4");
    }
}
