// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/

struct Solution {}

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<i32> = points.iter().map(|x| x[0]).collect();
        points.sort_unstable();
        points.dedup();

        let mut max_diff = 0;

        for i in 0..points.len()-1 {
            if points[i + 1] - points[i] > max_diff {
                max_diff = points[i + 1] - points[i];
            }
        }

        max_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![8, 7],
                vec![9, 9],
                vec![7, 4],
                vec![9, 7]
            ]),
            1,
            "Test 1"
        );

        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8]
            ]),
            3,
            "Test 2"
        );
    }
}
