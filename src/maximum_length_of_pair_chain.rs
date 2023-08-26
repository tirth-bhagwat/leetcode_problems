// https://leetcode.com/problems/maximum-length-of-pair-chain/

struct Solution {}

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by(|a, b| {
            if a[1] == b[1] {
                return a[0].cmp(&b[0]);
            }
            return a[1].cmp(&b[1]);
        });

        let mut count = 1;
        let mut prev_val = pairs[0][1];
        for i in &pairs[1..] {
            if prev_val < i[0] {
                count += 1;
                prev_val = i[1];
            }
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2,
            "Test 1"
        );
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
            3,
            "Test 2"
        );
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2,
            "Test 3"
        );
        assert_eq!(
            Solution::find_longest_chain(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6]
            ]),
            3,
            "Test 4"
        );
    }
}
