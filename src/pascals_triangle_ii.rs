// https://leetcode.com/problems/pascals-triangle-ii/

struct Solution {}
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1];

        for i in 1..row_index + 1 {
            let mut tmp = vec![0];
            tmp.append(row.clone().as_mut());
            row.push(0);
            row = row.iter().zip(tmp.iter()).map(|(a, b)| a + b).collect();
        }
        return row;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::get_row(0), vec![1], "Test 1");
        assert_eq!(Solution::get_row(1), vec![1, 1], "Test 2");
        assert_eq!(Solution::get_row(2), vec![1, 2, 1], "Test 3");
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1], "Test 4");
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1], "Test 5");
    }
}
