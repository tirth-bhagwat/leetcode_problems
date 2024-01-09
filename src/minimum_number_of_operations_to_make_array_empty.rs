// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/

struct Solution {}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut count = 1;
        let mut prev = nums[0];
        let mut res = 0;

        for i in &nums[1..] {
            if i == &prev {
                count += 1;
                continue;
            }

            match count {
                1 => {
                    return -1;
                }
                2 => res += 1,
                _ => {
                    res += match count % 3 {
                        1 => (count + 2) / 3,
                        2 => (count + 1) / 3,
                        0 => count / 3,
                        _ => unreachable!(),
                    }
                }
            }

            prev = *i;
            count = 1;
        }

        match count {
            1 => -1,
            2 => res + 1,
            _ => {
                res + match count % 3 {
                    1 => (count + 2) / 3,
                    2 => (count + 1) / 3,
                    0 => count / 3,
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_operations(vec![2, 1, 2, 2, 3, 3]), -1)
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_operations(vec![13, 7, 13, 7, 13, 7, 13, 13, 7]),
            4
        )
    }
}
