// https://leetcode.com/problems/unique-paths-ii/

struct Solution {}
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let mut grid = vec![vec![1; n as usize]; m as usize];
        let mut pos = -1;

        for i in 0..n {
            if obstacle_grid[0][i] == 1 || pos > -1 {
                if pos == -1 {
                    pos = i as i32;
                }
                grid[0][i] = 0;
            }
        }

        for j in 1..m {
            if pos == 0 {
                grid[j][0] = 0;
                continue;
            } else if j == 1 {
                pos = -1;
            }
            if obstacle_grid[j][0] == 1 || pos > -1 {
                if pos == -1 {
                    pos = j as i32;
                }
                grid[j][0] = 0;
            }
        }

        for i in 1..m as usize {
            for j in 1..n as usize {
                if obstacle_grid[i][j] == 1 {
                    grid[i][j] = 0;
                    continue;
                }

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
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2,
            "Test 1"
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1,
            "Test 2"
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![1, 0]]),
            0,
            "Test 3"
        );
        // [[0,0],[1,1],[0,0]]
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            0,
            "Test 4"
        )
    }
}
