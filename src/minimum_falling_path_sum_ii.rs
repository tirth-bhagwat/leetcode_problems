// https://leetcode.com/problems/minimum-falling-path-sum-ii/

struct Solution {}

use std::cmp::{max, min};
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut mins: Vec<((i32, usize), (i32, usize))> =
            matrix.iter().map(|x| Self::get_mins(x)).collect();

        let end = matrix[0].len() - 1;

        let mut r1 = mins[end].0 .0;
        let mut first_used = true;
        println!("res: {}, first_used: {}", r1, first_used);

        for i in (0..end).rev() {
            if first_used && mins[i].0 .1 == mins[i + 1].0 .1 {
                r1 += mins[i].1 .0;
                // println!("  a: {}", mins[i].1 .0);
                first_used = false;
                println!(
                    "  res: {}, sel: {}, first_used: {}",
                    r1, mins[i].1 .0, first_used
                );
            }
            // else if mins[i].0 .1 == mins[i + 1].1 .1 {
            //     r1 += mins[i].0 .0;
            //     // println!("  b: {}", mins[i].1 .0);
            //     first_used = true;
            //     println!("  res: {}, sel: {}, first_used: {}", r1, mins[i].0 .0, first_used);
            // }
            else {
                r1 += mins[i].0 .0;
                // println!("  c: {}", mins[i].0 .0);
                first_used = true;
                println!(
                    "  res: {}, sel: {}, first_used: {}",
                    r1,
                    min(mins[i].0 .0, mins[i].1 .0),
                    first_used
                );
            }
        }

        let mut r2 = mins[0].0 .0;
        let mut first_used = true;
        println!("res: {}, first_used: {}", r2, first_used);

        for i in 1..=end {
            if first_used && mins[i].0 .1 == mins[i - 1].0 .1 {
                r2 += mins[i].1 .0;
                // println!("  a: {}", mins[i].1 .0);
                first_used = false;
                println!(
                    "  res: {}, sel: {}, first_used: {}",
                    r2, mins[i].1 .0, first_used
                );
            }
            // else if mins[i].0 .1 == mins[i - 1].1 .1 {
            //     r2 += mins[i].0 .0;
            //     // println!("  b: {}", mins[i].1 .0);
            //     first_used = true;
            //     println!("  res: {}, sel: {}, first_used: {}", r2, mins[i].0 .0, first_used);
            // }
            else {
                r2 += mins[i].0 .0;
                // println!("  c: {}", mins[i].0 .0);
                first_used = true;
                println!(
                    "  res: {}, sel: {}, first_used: {}",
                    r2,
                    min(mins[i].0 .0, mins[i].1 .0),
                    first_used
                );
            }
        }

        min(r1, r2)
    }

    pub fn get_mins(arr: &Vec<i32>) -> ((i32, usize), (i32, usize)) {
        if arr.len() == 1 {
            return ((arr[0], 0), (arr[0], 0));
        }

        let ((mut a, mut ind_a), (mut b, mut ind_b)) = if arr[0] > arr[1] {
            ((arr[1], 1), (arr[0], 0))
        } else {
            ((arr[0], 0), (arr[1], 1))
        };

        for i in 2..arr.len() {
            if arr[i] < b {
                if arr[i] < a {
                    b = a;
                    ind_b = ind_a;
                    a = arr[i];
                    ind_a = i;
                } else {
                    b = arr[i];
                    ind_b = i;
                }
            }
        }

        ((a, ind_a), (b, ind_b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // [[-73,61,43,-48,-36],[3,30,27,57,10],[96,-76,84,59,-15],[5,-49,76,31,-7],[97,91,61,-46,67]]

        let mut matrix = vec![
            vec![-73, 61, 43, -48, -36],
            vec![3, 30, 27, 57, 10],
            vec![96, -76, 84, 59, -15],
            vec![5, -49, 76, 31, -7],
            vec![97, 91, 61, -46, 67],
        ];
        assert_eq!(Solution::min_falling_path_sum(matrix), -192, "Test 1");

        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]),
            13,
            "Test 2"
        );

        assert_eq!(Solution::min_falling_path_sum(vec![vec![7],]), 7, "Test 3")
    }

    #[test]
    fn test2() {
        // [[-37,51,-36,34,-22],[82,4,30,14,38],[-68,-52,-92,65,-85],[-49,-3,-77,8,-19],[-60,-71,-21,-62,-73]]

        let mut matrix = vec![
            vec![-37, 51, -36, 34, -22],
            vec![82, 4, 30, 14, 38],
            vec![-68, -52, -92, 65, -85],
            vec![-49, -3, -77, 8, -19],
            vec![-60, -71, -21, -62, -73],
        ];
        assert_eq!(Solution::min_falling_path_sum(matrix), -268, "Test 4");
    }

    #[test]
    fn test3() {
        // [[50,-18,-38,39,-20,-37,-61,72,22,79],[82,26,30,-96,-1,28,87,94,34,-89],[55,-50,20,76,-50,59,-58,85,83,-83],[39,65,-68,89,-62,-53,74,2,-70,-90],[1,57,-70,83,-91,-32,-13,49,-11,58],[-55,83,60,-12,-90,-37,-36,-27,-19,-6],[76,-53,78,90,70,62,-81,-94,-32,-57],[-32,-85,81,25,80,90,-24,10,27,-55],[39,54,39,34,-45,17,-2,-61,-81,85],[-77,65,76,92,21,68,78,-13,39,22]]
    }
}
