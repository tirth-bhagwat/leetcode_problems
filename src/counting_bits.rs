// https://leetcode.com/problems/counting-bits/

struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0, 1];
        match n {
            0 => return res[..=0].to_vec(),
            1 => return res,
            _ => {
                let mut p = 1;
                let mut val = 2;
                let mut next_val = 2_i32.pow(p as u32 + 1);

                for i in 2..=n {
                    res.push(res[i as usize - val] + 1);
                    if i + 1 == next_val {
                        p += 1;
                        val = next_val as usize;
                        next_val = 2_i32.pow(p as u32 + 1);
                    }
                }
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
        assert_eq!(Solution::count_bits(6), vec![0, 1, 1, 2, 1, 2, 2]);
        assert_eq!(
            Solution::count_bits(10),
            vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2]
        );
        assert_eq!(Solution::count_bits(0), vec![0]);
        assert_eq!(Solution::count_bits(1), vec![0, 1]);
    }
}
