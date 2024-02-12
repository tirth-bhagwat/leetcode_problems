// https://leetcode.com/problems/majority-element/

struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut num = nums[0];
        let mut count = 1;

        for i in nums.iter().skip(1) {
            if *i == num {
                count += 1;
            } else {
                count -= 1;
            }

            if count < 0 {
                num = *i;
                count = 1;
            }
        }

        return num;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // Positive Cases
    #[test]
    fn basic_majority() {
        let nums = vec![3, 2, 3];
        let expected = 3;
        assert_eq!(Solution::majority_element(nums), expected);
    }

    #[test]
    fn majority_at_beginning() {
        let nums = vec![3, 3, 3, 2, 1];
        let expected = 3;
        assert_eq!(Solution::majority_element(nums), expected);
    }

    #[test]
    fn majority_at_end() {
        let nums = vec![2, 1, 3, 3, 3];
        let expected = 3;
        assert_eq!(Solution::majority_element(nums), expected);
    }



    #[test]
    fn single_element() {
        let nums = vec![3];
        let expected = 3;
        assert_eq!(Solution::majority_element(nums), expected);
    }


    #[test]
    fn size_lower_bound() {
        let nums = vec![1];
        let expected = 1;
        assert_eq!(Solution::majority_element(nums), expected);
    }

    #[test]
    fn size_upper_bound() {
        let nums = vec![1; 50000]; // Repeat element to reach upper bound
        let expected = 1;
        assert_eq!(Solution::majority_element(nums), expected);
    }

    // Tricky Cases
    #[test]
    fn exactly_half() {
        let nums = vec![2, 2, 3, 3, 3];
        let expected = 3;
        assert_eq!(Solution::majority_element(nums), expected);
    }

    #[test]
    fn multiple_minorities() {
        let nums = vec![1, 1, 2, 2, 2, 3, 3, 3, 3];
        let expected = 3;
        assert_eq!(Solution::majority_element(nums), expected);
    }
}
