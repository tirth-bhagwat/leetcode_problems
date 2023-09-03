// https://leetcode.com/problems/unique-paths/

struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut grid = vec![vec![1; n as usize]; m as usize];
        for i in 1..m as usize {
            for j in 1..n as usize {
                grid[i][j] = grid[i][j - 1] + grid[i - 1][j];
            }
        }

        return grid[m as usize - 1][n as usize - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::unique_paths(3, 7), 28, "Test 1");
        assert_eq!(Solution::unique_paths(3, 2), 3, "Test 2");
        assert_eq!(Solution::unique_paths(7, 3), 28, "Test 3");
        assert_eq!(Solution::unique_paths(3, 3), 6, "Test 4");
        assert_eq!(Solution::unique_paths(1, 1), 1, "Test 5");
        assert_eq!(Solution::unique_paths(1, 2), 1, "Test 6");
        assert_eq!(Solution::unique_paths(2, 1), 1, "Test 7");
    }
}
