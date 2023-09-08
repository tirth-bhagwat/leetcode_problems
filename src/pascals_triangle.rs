// https://leetcode.com/problems/pascals-triangle/

struct Solution {}
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut pascal: Vec<Vec<i32>> = vec![vec![1]];
        for i in (2..=num_rows).rev() {
            match num_rows {
                2 => {
                    &mut pascal.push(vec![1, 1]);
                }
                _ => {
                    let mut res = vec![1];
                    // let mut pascal = Solution::generate(num_rows - 1);
                    let prev = pascal.last().unwrap();

                    for i in 0..prev.len() - 1 {
                        res.push(prev[i] + prev[i + 1]);
                    }

                    res.push(1);
                    &mut pascal.push(res);
                }
            }
        }

        return pascal;
    }
}
