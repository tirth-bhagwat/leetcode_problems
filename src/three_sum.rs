// https://leetcode.com/problems/3sum/

use std::collections::HashSet;
struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut pos = vec![];
        let mut neg_mod = vec![];
        let mut zeros = 0;
        let mut res: HashSet<Vec<i32>> = HashSet::new();

        for i in nums {
            if i > 0 {
                pos.push(i)
            } else if i < 0 {
                neg_mod.push(i.abs());
            } else {
                zeros += 1;
            }
        }

        if zeros >= 3 {
            res.insert(vec![0, 0, 0]);
        }
        if pos.len() == 0 || neg_mod.len() == 0 {
            return res.into_iter().collect();
        }

        pos.sort_unstable();
        neg_mod.sort_unstable();

        if zeros >= 1 {
            pos.insert(0, 0);
            neg_mod.insert(0, 0);
        }

        for i in &pos {
            let lst = Self::two_sum(&neg_mod, &i);
            for x in lst {
                if x.1 != -1 && x.0 != -1 {
                    let mut tmp = vec![*i, -x.0, -x.1];
                    tmp.sort_unstable();
                    res.insert(tmp);
                }
            }
        }

        for i in &neg_mod {
            let lst = Self::two_sum(&pos, &i);
            for x in lst {
                if x.1 != -1 && x.0 != -1 {
                    let mut tmp = vec![-i, x.0, x.1];
                    tmp.sort_unstable();
                    res.insert(tmp);
                }
            }
        }

        return res.into_iter().collect();
    }

    pub fn two_sum(numbers: &Vec<i32>, target: &i32) -> Vec<(i32, i32)> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        let mut res = vec![(-1, -1)];

        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == *target {
                res.push((numbers[left], numbers[right]));
                left += 1;
                right -= 1;
            } else if sum < *target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);

        assert!(false)
    }
}
