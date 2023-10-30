// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/

struct Solution {}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        let mut x = arr
            .iter()
            .map(|x| (*x, Solution::get_num_1(*x)))
            .collect::<Vec<(i32, i32)>>();
        x.sort_by(|x, y| {
            if x.1 == y.1 {
                return x.0.cmp(&y.0);
            }
            x.1.cmp(&y.1)
        });

        x.iter().map(|x| x.0).collect()
    }

    pub fn get_num_1(mut num: i32) -> i32 {
        let mut count = 0;
        while num != 0 {
            count += num & 1;
            num = num >> 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_1() {
        let solutions = vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1];
        for i in 0..solutions.len() {
            assert_eq!(solutions[i], Solution::get_num_1(i as i32), "Number {}", i);
        }
    }

    #[test]
    fn test_sort_by_bits() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7],
            "Test 1"
        );

        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
            "Test 2"
        );
    }
}
