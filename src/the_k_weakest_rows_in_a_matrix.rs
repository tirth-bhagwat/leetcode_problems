// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/

struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut x: Vec<(usize, i32)> = mat
            .iter()
            .enumerate()
            .map(|(i, val)| -> (usize, i32) { (i, val.iter().sum()) })
            .collect::<Vec<(usize, i32)>>();
        x.sort_by(|a, b| {
            if a.1 != b.1 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        return x.iter().take(k as usize).map(|(a, b)| *a as i32).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            ),
            vec![2, 0, 3]
        );
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            ),
            vec![0, 2]
        );
    }
}
