// https://leetcode.com/problems/find-pivot-index/

struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut l_sum = 0;
        let mut r_sum: i32 = (&nums[1..]).iter().sum();
        let mut prev = nums[0];

        println!("l_sum: {}, r_sum: {}", l_sum, r_sum);
        if r_sum == l_sum {
            return 0;
        }

        for (i, n) in nums[1..].iter().enumerate() {
            r_sum -= n;
            l_sum += prev;
            prev = *n;

            println!("i: {}, l_sum: {}, r_sum: {}", i, l_sum, r_sum);

            if r_sum == l_sum {
                return i as i32 + 1;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3, "Test 1");
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1, "Test 2");
        assert_eq!(Solution::pivot_index(vec![2, 1, - 1]), 0, "Test 3");
    }
}
