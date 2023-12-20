
// https://leetcode.com/problems/buy-two-chocolates/

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort_unstable();
        return match (prices[0] + prices[1]).cmp(&money) {
            Ordering::Greater => money,
            _ => money - prices[0] - prices[1]
        }
    }
}


