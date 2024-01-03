// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/

use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hm = [0_usize; 200];

        for i in nums {
            hm[i as usize - 1] += 1;
        }

        let mut hm = hm
            .iter()
            .enumerate()
            .filter_map(|(k, val)| {
                if val != &0 {
                    return Some((k + 1, *val));
                }
                None
            })
            .collect::<Vec<(usize, usize)>>();

        hm.sort_unstable_by_key(|x| Reverse(x.1));

        let mut res = vec![];
        let num = hm[0].0;
        for i in 0..hm[0].1 {
            res.push(vec![num as i32]);
        }

        for (num, rep) in hm.into_iter().skip(1) {
            for j in 0..rep {
                res[j].push(num as i32);
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
        assert_eq!(
            Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
            vec![vec![1, 3, 4], vec![1, 2], vec![1]],
            "Test 1"
        );

        assert_eq!(
            Solution::find_matrix(vec![1, 1, 1, 1]),
            vec![vec![1], vec![1], vec![1], vec![1]],
            "Test 2"
        );

        assert_eq!(
            Solution::find_matrix(vec![2, 1, 1]),
            vec![vec![2, 1], vec![1]],
            "Test 3"
        );
    }
}
