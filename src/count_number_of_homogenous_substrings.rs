// https://leetcode.com/problems/count-number-of-homogenous-substrings/

struct Solution {}
const MOD: u64 = 1000000007;
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut res = 0;
        let mut curr = 1;
        let mut prev = s.chars().nth(0).unwrap();

        for i in s.chars().skip(1) {
            if i == prev {
                curr += 1;
            } else {
                res = res + Solution::sum_n(curr) % MOD;
                curr = 1;
            }

            prev = i;
        }
        res = res + Solution::sum_n(curr) % MOD;

        res as i32
    }

    pub fn sum_n(n: u64) -> u64 {
        (n * (n + 1) / 2) % MOD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::count_homogenous("abbcccaa".to_string()),
            13,
            "Test 1"
        );
        assert_eq!(Solution::count_homogenous("xy".to_string()), 2, "Test 2");
        assert_eq!(
            Solution::count_homogenous("zzzzz".to_string()),
            15,
            "Test 3"
        );
    }
}
