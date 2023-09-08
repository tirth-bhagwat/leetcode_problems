// https://leetcode.com/problems/calculate-digit-sum-of-a-string/

struct Solution {}

impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let mut x = "".to_string();

        while s.len() > k as usize {
            let mut i = s.chars().into_iter();
            let mut size = s.len() / k as usize;
            if s.len() % k as usize != 0 {
                size += 1
            }
            for _ in 0..size {
                x.push_str(
                    (&mut i)
                        .take(k as usize)
                        .fold(0, |acc, a| acc + a as i32 - 48)
                        .to_string()
                        .as_ref(),
                );
            }
            s = x;
            x = "".to_string();
        }

        return s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::digit_sum("1234".to_string(), 3), "64");
        assert_eq!(Solution::digit_sum("1234".to_string(), 2), "37");
        assert_eq!(Solution::digit_sum("11111222223".to_string(), 3), "135");
        assert_eq!(Solution::digit_sum("00000000".to_string(), 3), "000");
    }
}
