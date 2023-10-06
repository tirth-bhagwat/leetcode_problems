// https://leetcode.com/problems/integer-break/

struct Solution {}

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            _ => {
                let prod = match n % 3 {
                    0 => 9,
                    1 => 12,
                    2 => 6,
                    _ => {
                        unreachable!()
                    }
                };

                let sum = match prod {
                    9 => 6,
                    12 => 7,
                    6 => 5,
                    _ => {
                        unreachable!()
                    }
                };

                prod as i32 * (3_i32.pow((n as u32 - sum) / 3))
            }
        }

        // if initial != 0 {
        //     return initial;
        // }
        //
        // let prod = match n % 3 {
        //     0 => 9,
        //     1 => 12,
        //     2 => 6,
        //     _ => {
        //         unreachable!()
        //     }
        // };
        //
        // let sum = match prod {
        //     9 => 6,
        //     12 => 7,
        //     6 => 5,
        //     _ => {
        //         unreachable!()
        //     }
        // };
        //
        // prod as i32 * (3_i32.pow((n as u32 - sum) / 3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::integer_break(2), 1, "Test 1");
        assert_eq!(Solution::integer_break(3), 2, "Test 2");
        assert_eq!(Solution::integer_break(4), 4, "Test 3");
        assert_eq!(Solution::integer_break(8), 18, "Test 4");
        assert_eq!(Solution::integer_break(9), 27, "Test 4");
        assert_eq!(Solution::integer_break(10), 36, "Test 5");
    }
}
