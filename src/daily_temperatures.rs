// https://leetcode.com/problems/daily-temperatures/

struct Solution {}

impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut occurrence = [i32::MAX; 71];

        // let not_assigned :Vec<usize>=vec![0];
        // let prev_temp = temperatures[0];
        let mut max_temp = *temperatures.last().unwrap() as usize;
        let mut curr = 0;
        for i in (0..temperatures.len()).rev() {
            curr = temperatures[i] as usize;
            occurrence[curr - 30] = i as i32;

            if curr >= max_temp {
                max_temp = curr;
                temperatures[i] = 0;
                continue;
            }

            let min_ind = *occurrence[curr - 30 + 1..=max_temp - 30]
                .iter()
                .min()
                .unwrap();
            temperatures[i] = min_ind - i as i32;
        }
        temperatures
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        )
    }
}
