// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/

struct Solution {}

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        println!("{:?}", &nums);

        for i in nums.chunks(3) {
            if i[1] - i[0] > k || i[2] - i[1] > k || i[2] - i[0] > k {
                return vec![];
            }
        }

        nums.chunks(3)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<i32>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // [15,13,12,13,12,14,12,2,3,13,12,14,14,13,5,12,12,2,13,2,2]
    // output: []

    #[test]
    fn test() {
        assert_eq!(
            Solution::divide_array(
                vec![15, 13, 12, 13, 12, 14, 12, 2, 3, 13, 12, 14, 14, 13, 5, 12, 12, 2, 13, 2, 2],
                2
            ),
            vec![vec![]]
        );
    }
}
