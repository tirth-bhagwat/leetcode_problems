// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut max_price = *prices.last().unwrap();

        for i in (0..prices.len()).rev() {
            if prices[i] < max_price {
                max_profit = max(max_profit, max_price - prices[i]);
            } else {
                max_price = prices[i];
            }
        }

        max_profit
    }
}
