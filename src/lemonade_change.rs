// https://leetcode.com/problems/lemonade-change/

struct Solution {}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five: u32 = 0;
        let mut ten: u32 = 0;
        let mut twenty: u32 = 0;

        if bills[0] != 5 {
            return false;
        }

        for i in bills.iter().skip(1) {
            match i {
                5 => {
                    five += 1;
                }
                10 => {
                    ten += 1;
                    if five == 0 {
                        return false;
                    }
                    five -= 1;
                }
                20 => {
                    twenty += 1;
                    if five >= 3 {
                        if ten == 0{
                            five -= 3;
                            continue
                        }
                        five -= 1;
                        ten -= 1;
                        continue
                    } else if ten >= 1 {
                        five -= 1;
                        ten -= 1;
                        continue
                    }
                    return false;
                }
                _ => panic!("Fake note :)"),
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::lemonade_change(vec![5, 5, 5, 10, 20]),
            true,
            "Test 1"
        );
        assert_eq!(
            Solution::lemonade_change(vec![5, 5, 10, 10, 20]),
            false,
            "Test 2"
        );
    }
}
