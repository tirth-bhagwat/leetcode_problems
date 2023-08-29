// https://leetcode.com/problems/minimum-penalty-for-a-shop/

struct Solution {}

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut curr_penalty = customers.chars().into_iter().filter(|x| x.eq(&'Y')).count() as i32;
        let mut min_penalty = curr_penalty;
        let mut at_hour = 0;

        for (i, j) in customers.chars().into_iter().enumerate() {
            if j == 'N' {
                curr_penalty += 1;
            } else {
                curr_penalty -= 1;
            }

            if curr_penalty < min_penalty {
                min_penalty = curr_penalty;
                at_hour = i as i32 + 1;
            } else if curr_penalty == min_penalty {
                if (i as i32) < at_hour {
                    at_hour = i as i32;
                }
            }
        }

        return at_hour;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::best_closing_time("".to_string()), 0);
        assert_eq!(Solution::best_closing_time("YYNY".to_string()), 2);
        assert_eq!(Solution::best_closing_time("NNNN".to_string()), 0);
        assert_eq!(Solution::best_closing_time("YYYY".to_string()), 4);
    }
}
