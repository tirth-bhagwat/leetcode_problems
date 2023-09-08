// https://leetcode.com/problems/find-triangular-sum-of-an-array/

struct Solution {}

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while (&nums).len() != 1 {
            let mut tmp = (&nums).clone();
            tmp.rotate_right(1);
            tmp[0] = 0;

            nums = nums
                .iter()
                .zip(tmp.iter())
                .map(|(a, b)| (a + b) % 10)
                .collect::<Vec<i32>>()[1..]
                .to_vec();

        }

        return nums[0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::triangular_sum(vec![1, 2, 3]), 8, "Test 1");
        assert_eq!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]), 8, "Test 2");
        assert_eq!(Solution::triangular_sum(vec![3, 4, 5]), 6, "Test 3");
        assert_eq!(Solution::triangular_sum(vec![5]), 5, "Test 4");
    }
}
