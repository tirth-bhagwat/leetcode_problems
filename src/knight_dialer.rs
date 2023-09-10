// https://leetcode.com/problems/knight-dialer/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    const MOD: i64 = 1_000_000_007;
    pub fn knight_dialer(n: i32) -> i32 {
        let can_travel: Vec<Vec<usize>> = vec![
            vec![4, 6],    // 0
            vec![6, 8],    // 1
            vec![7, 9],    // 2
            vec![4, 8],    // 3
            vec![3, 9, 0], // 4
            vec![],        // 5
            vec![1, 7, 0], // 6
            vec![2, 6],    // 7
            vec![1, 3],    // 8
            vec![2, 4],    // 9
        ];

        let mut prev_data: Vec<HashMap<i32, i64>> = vec![HashMap::new(); 10];

        let mut res: i64 = 0;
        let x = (2, 4);

        for i in 0..10 {
            println!("i = {}", i);
            res += Solution::count_for_pos(n - 1, i, &can_travel, &mut prev_data) % Solution::MOD;
        }

        return (res % Solution::MOD) as i32;
    }

    pub fn count_for_pos(
        dist: i32,
        pos: usize, 
        travel_list: &Vec<Vec<usize>>,
        prev_data: &mut Vec<HashMap<i32, i64>>,
    ) -> i64 {
        if dist == 0 {
            return 1;
        }

        if let Some(val) = prev_data[pos].get(&dist) {
            return *val;
        }

        let mut res = 0;
        for i in &travel_list[pos] {
            res += Solution::count_for_pos(dist - 1, *i, &travel_list, prev_data) % Solution::MOD;
        }

        prev_data[pos].insert(dist, res);

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::knight_dialer(1), 10, "Test 1");
        assert_eq!(Solution::knight_dialer(2), 20, "Test 2");
        assert_eq!(Solution::knight_dialer(3), 46, "Test 3");
        // assert_eq!(Solution::knight_dialer(4), 103, "Test 4");
        // assert_eq!(Solution::knight_dialer(5), 230, "Test 5");
        // assert_eq!(Solution::knight_dialer(6), 603, "Test 6");
        assert_eq!(Solution::knight_dialer(3131), 136006598, "Test 4");
    }
}
