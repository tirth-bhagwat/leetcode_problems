// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, val) in group_sizes.iter().enumerate() {
            if let Some(x) = groups.get_mut(val) {
                x.push(i);
            } else {
                groups.insert(*val, vec![i]);
            }
        }

        let mut res = Vec::<Vec<i32>>::new();

        for i in groups.keys() {
            let a = groups.get(i).unwrap();
            let mut iterator = a.iter();
            for _ in 0..a.len() / *i as usize {
                res.push(
                    (&mut iterator)
                        .take(*i as usize)
                        .map(|a| *a as i32)
                        .collect(),
                );
            }
        }
        res.sort_by(|a, b| a.len().cmp(&b.len()));
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
        );
        assert_eq!(
            Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
            vec![vec![1], vec![0, 5], vec![2, 3, 4]]
        );
    }
}
