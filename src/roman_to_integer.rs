// https://leetcode.com/problems/roman-to-integer/
struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut prev = 0;
        let mut curr = 0;
        let mut sum = 0;
        for i in s.chars().rev().into_iter() {
            curr = match i {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };
            println!("{}", curr);
            if curr < prev {
                println!("{} > {}", curr, prev);
                sum -= curr;
            } else {
                println!("{} <= {}", curr, prev);
                sum += curr;
            }
            prev = curr;
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        // 3, 58, 1994, 203, 909, 999, 1000, 1001, 2000

        assert_eq!(Solution::roman_to_int("III".to_string()), 3, "Test 1");
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58, "Test 2");
        assert_eq!(
            Solution::roman_to_int("MCMXCIV".to_string()),
            1994,
            "Test 3"
        );
        assert_eq!(Solution::roman_to_int("CCIII".to_string()), 203, "Test 4");
        assert_eq!(Solution::roman_to_int("CMXCIX".to_string()), 999, "Test 5");
        assert_eq!(Solution::roman_to_int("M".to_string()), 1000, "Test 6");
        assert_eq!(Solution::roman_to_int("MI".to_string()), 1001, "Test 7");
        assert_eq!(Solution::roman_to_int("MM".to_string()), 2000, "Test 8");
    }
}
