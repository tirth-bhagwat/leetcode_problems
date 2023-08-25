// https://leetcode.com/problems/integer-to-roman/

use std::convert::TryInto;
struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        match num {
            n if n > 999 => {
                return vec!["M"; (n / 1000).try_into().unwrap()].concat()
                    + Solution::int_to_roman(n % 1000).as_ref();
            }
            n if n > 99 => {
                return Solution::create_roman((n / 100).try_into().unwrap(), "CDM".to_string())
                    + Solution::int_to_roman(n % 100).as_ref();
            }
            n if n > 9 => {
                return Solution::create_roman((n / 10).try_into().unwrap(), "XLC".to_string())
                    + Solution::int_to_roman(n % 10).as_ref();
            }
            n if n >= 0 => {
                return Solution::create_roman((n).try_into().unwrap(), "IVX".to_string());
            }
            _ => {
                panic!("Invalud number")
            }
        }
    }

    pub fn create_roman(num: i32, guide: String) -> String {
        match num {
            n if n == 1 => {
                return guide.get(0..1).unwrap().to_string();
            }
            n if n == 2 => {
                return guide.get(0..1).unwrap().to_string() + guide.get(0..1).unwrap();
            }
            n if n == 3 => {
                return guide.get(0..1).unwrap().to_string()
                    + guide.get(0..1).unwrap()
                    + guide.get(0..1).unwrap();
            }
            n if n == 4 => {
                return guide.get(0..1).unwrap().to_string() + guide.get(1..2).unwrap();
            }
            n if n == 5 => {
                return guide.get(1..2).unwrap().to_string();
            }
            n if n == 6 => {
                return guide.get(1..2).unwrap().to_string() + guide.get(0..1).unwrap();
            }
            n if n == 7 => {
                return guide.get(1..2).unwrap().to_string()
                    + guide.get(0..1).unwrap()
                    + guide.get(0..1).unwrap();
            }
            n if n == 8 => {
                return guide.get(1..2).unwrap().to_string()
                    + guide.get(0..1).unwrap()
                    + guide.get(0..1).unwrap()
                    + guide.get(0..1).unwrap();
            }
            n if n == 9 => {
                return guide.get(0..1).unwrap().to_string() + guide.get(2..3).unwrap();
            }
            n if n == 0 => {
                return "".to_string();
                // return guide.get(2..3).unwrap().to_string();
            }
            _ => {
                panic!("Invalid roman number");
            }
        }

        return todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string(), "Test 1");
        assert_eq!(Solution::int_to_roman(4), "IV".to_string(), "Test 2");
        assert_eq!(Solution::int_to_roman(9), "IX".to_string(), "Test 3");
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string(), "Test 4");
        assert_eq!(
            Solution::int_to_roman(1994),
            "MCMXCIV".to_string(),
            "Test 5"
        );
        assert_eq!(Solution::int_to_roman(1), "I".to_string(), "Test 6");
        assert_eq!(
            Solution::int_to_roman(3999),
            "MMMCMXCIX".to_string(),
            "Test 7"
        );

        assert_eq!(Solution::int_to_roman(1000), "M".to_string(), "Test 8");
        assert_eq!(Solution::int_to_roman(100), "C".to_string(), "Test 9");
        assert_eq!(Solution::int_to_roman(10), "X".to_string(), "Test 10");
        assert_eq!(Solution::int_to_roman(1), "I".to_string(), "Test 11");
    }
}
