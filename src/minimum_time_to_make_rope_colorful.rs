// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/

struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.as_bytes().to_vec();
        let mut time = 0;
        let mut i = 0;

        while i < colors.len() {
            if (i < colors.len() - 1 && colors[i] != colors[i + 1]) || i == colors.len() - 1 {
                i += 1;
                continue;
            }

            let start = i;
            while i < colors.len() - 1 && colors[i] == colors[i + 1] {
                i += 1;
            }

            let mut times = needed_time[start..i + 1].to_vec();
            times.sort();
            time += times.iter().take(times.len() - 1).fold(0, |acc, x| acc + x);
        }

        time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_cost(String::from("abaac"), vec![1, 2, 3, 4, 5]),
            3,
            "Test 1"
        );

        assert_eq!(
            Solution::min_cost(String::from("abc"), vec![1, 2, 3]),
            0,
            "Test 2"
        );

        assert_eq!(
            Solution::min_cost(String::from("aabaa"), vec![1, 2, 3, 4, 1]),
            2,
            "Test 3"
        );
    }
}
