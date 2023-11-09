// https://leetcode.com/problems/number-of-substrings-with-only-1s/

struct Solution {}
const MOD: u64 = 1000000007;
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        s.split('0')
            .filter_map(|x| match x.len() {
                0 => None,
                _ => Some(Solution::sum_n(x.len() as u64)),
            })
            .fold(0, |sum, num| sum + (num % MOD) as i32)
    }

    pub fn sum_n(n: u64) -> u64 {
        (n * (n + 1) / 2) % MOD
    }
}
