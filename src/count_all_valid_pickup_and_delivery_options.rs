// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/

struct Solution {}

impl Solution {
    const MOD: i64 = 1_000_000_007;
    pub fn count_orders(n: i32) -> i32 {
        let mut prev: i64 = 1;

        for i in 1_i64..n as i64 {
            let n = (2 * i) + 1;
            let new_moves = ((n * (n + 1)) / 2) - 1;
            prev = (((new_moves * prev) + prev) as i64) % Solution::MOD;
        }
        return prev.try_into().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::count_orders(1), 1, "Test 1");
        assert_eq!(Solution::count_orders(2), 6, "Test 2");
        assert_eq!(Solution::count_orders(3), 90, "Test 3");
        assert_eq!(Solution::count_orders(4), 2520, "Test 4");
        assert_eq!(Solution::count_orders(8), 729647433, "Test 5");
    }
}
