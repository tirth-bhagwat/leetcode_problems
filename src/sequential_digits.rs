// https://leetcode.com/problems/sequential-digits/

use std::cmp::min;

struct Solution {}
impl Solution {
    pub fn sequential_digits(low: i32, mut high: i32) -> Vec<i32> {
        let mut res = vec![];
        let max_possible: u64 = 123456789;
        let max_adder: u64 = 111111111;
        if low > max_possible as i32 {
            return res;
        }
        high = min(high, max_possible as i32);

        let low_len = low.to_string().len();
        let high_len = high.to_string().len();

        for i in low_len..=high_len {
            let multiplier_min = 10_u64.pow((9 - i) as u32);
            let mut start = max_possible / multiplier_min;

            let multiplier_max = 10_u64.pow(i as u32);
            let mut end = max_possible - ((max_possible / multiplier_max) * multiplier_max);

            let adder = max_adder / multiplier_min;

            while start <= min(high as u64, end) {
                if start >= low as u64 {
                    res.push(start as i32);
                }
                start += adder;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::sequential_digits(58, 155),
            vec![12,23,34,45,56,67,78,89,123]
        )
    }
}
