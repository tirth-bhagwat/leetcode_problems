// https://leetcode.com/problems/number-of-good-pairs/

struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; 100];

        for i in nums {
            count[i as usize - 1] += 1;
        }

        count
            .iter()
            .filter(|x| **x > 1)
            .fold(0, |acc, x| acc + Solution::ncr(*x, 2) as i32)
    }

    pub fn ncr(n: usize, r: usize) -> usize {
        Solution::partial_fact(n, n - r + 1) / Solution::partial_fact(r, 1)
    }

    pub fn partial_fact(num: usize, from: usize) -> usize {
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
        assert_eq!(
            Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]),
            4,
            "Test 1"
        );
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6, "Test 2");
        assert_eq!(
            Solution::num_identical_pairs(vec![
                6, 5, 1, 5, 7, 7, 9, 1, 5, 7, 1, 6, 10, 9, 7, 4, 1, 8, 7, 1, 1, 8, 6, 4, 7, 4, 10,
                5, 3, 9, 10, 1, 9, 5, 5, 4, 1, 7, 4, 2, 9, 2, 6, 6, 4, 2, 10, 3, 5, 3, 6, 4, 7, 4,
                6, 4, 4, 6, 3, 4, 10, 1, 10, 6, 10, 4, 9, 6, 6, 4, 8, 6, 9, 5, 4,
            ]),
            303,
            "Test 3"
        );
        assert_eq!(
            Solution::num_identical_pairs(vec![
                2, 2, 1, 5, 1, 5, 5, 2, 3, 1, 1, 5, 3, 2, 3, 3, 3, 1, 3, 3, 4, 3, 1, 3, 1, 4, 5, 5,
                2, 2, 1, 3, 5, 2, 2, 4, 3, 2, 5, 3, 1, 1, 3, 3, 2, 5, 2, 1, 2, 4, 3, 4, 4, 3, 2, 4,
                4, 1, 3, 3, 3, 5, 5, 5, 4, 1, 1, 2, 3, 3, 2, 5, 3, 4, 5, 3, 1, 2, 5, 4, 5, 2, 3, 3,
                1, 5, 2, 4, 2, 4, 4, 3, 1, 3,
            ]),
            885,
            "Test 4"
        );
    }
}
