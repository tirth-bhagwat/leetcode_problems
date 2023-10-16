// https://leetcode.com/problems/pascals-triangle-ii/

struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![];

        for i in 0..=row_index {
            res.push(Solution::ncr(row_index as u128, i as u128) as i32);
        }
        return res;
    }

    pub fn ncr(n: u128, r: u128) -> u128 {
        if n - r > r {
            Solution::partial_fact(n, n - r + 1) / Solution::partial_fact(r, 1)
        } else {
            Solution::partial_fact(n, r + 1) / Solution::partial_fact(n - r, 1)
        }
    }

    pub fn partial_fact(num: u128, from: u128) -> u128 {
        let mut res = 1;
        for i in from..=num {
            res *= i;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::get_row(0), vec![1], "Test 1");
        assert_eq!(Solution::get_row(1), vec![1, 1], "Test 2");
        assert_eq!(Solution::get_row(2), vec![1, 2, 1], "Test 3");
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1], "Test 4");
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1], "Test 5");
        assert_eq!(
            Solution::get_row(18),
            vec![
                1, 18, 153, 816, 3060, 8568, 18564, 31824, 43758, 48620, 43758, 31824, 18564, 8568,
                3060, 816, 153, 18, 1
            ],
            "Test 6"
        );
    }
}
