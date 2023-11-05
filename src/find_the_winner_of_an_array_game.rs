// https://leetcode.com/problems/find-the-winner-of-an-array-game/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn get_winner(mut arr: Vec<i32>, k: i32) -> i32 {
        let a = arr.clone();
        let maxi = a.into_iter().max().unwrap();
        if k >= arr.len() as i32 {
            return maxi;
        }
        let mut win_count = 0;
        let mut winner = max(arr[0], arr[1]);

        while win_count < k {
            if arr[0] == maxi {
                return maxi;
            }

            if arr[0] > arr[1] {
                win_count += 1;
                let tmp = arr.remove(1);
                arr.push(tmp);
            } else {
                win_count = 1;
                winner = arr[1];
                let tmp = arr.remove(0);
                arr.push(tmp);
            }
        }

        winner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_winner(vec![3, 2, 1], 10), 3, "Test 1");

        assert_eq!(
            Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2),
            5,
            "Test 2"
        );
    }
}
