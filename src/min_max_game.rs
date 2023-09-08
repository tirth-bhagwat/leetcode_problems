// https://leetcode.com/problems/min-max-game/

struct Solution {}

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut min = true;
        while (&nums).len() != 1 {
            let mut i = nums.iter();
            nums = nums
                .iter()
                .map_while(|_| {
                    if min {
                        if let Some(x) = (&mut i).take(2).min() {
                            min = !min;
                            Some(*x)
                        } else {
                            None
                        }
                    } else {
                        if let Some(x) = (&mut i).take(2).max() {
                            min = !min;
                            Some(*x)
                        } else {
                            None
                        }
                    }
                })
                .collect();
        }

        return nums[0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]),
            1,
            "test 1"
        );
        assert_eq!(Solution::min_max_game(vec![1]), 1, "test 2");
    }
}
