// https://leetcode.com/problems/maximum-average-subarray-i/

struct Solution {}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = nums.get(0..k as usize).unwrap().iter().sum::<i32>() as f64;
        let mut max_sum = sum;
        for i in 1..nums.len() - k as usize + 1 {
            sum = sum + (-nums[i - 1] + nums[i + k as usize - 1]) as f64;
            if sum > max_sum {
                max_sum = sum;
            }
        }

        return max_sum as f64 / k as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75,
            "Test 1"
        );
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 2),
            26.5,
            "Test 1"
        );
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.00000, "Test 2");
        assert_eq!(
            Solution::find_max_average(vec![-1, -1], 1),
            -1.00000,
            "Test 3"
        );
        assert_eq!(
            Solution::find_max_average(vec![4, 2, 1, 3, 3], 2),
            3.00000,
            "Test 4"
        );
    }
}
