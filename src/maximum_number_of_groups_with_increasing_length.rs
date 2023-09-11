// https://leetcode.com/problems/maximum-number-of-groups-with-increasing-length/

struct Solution {}

impl Solution {
    pub fn max_increasing_groups(usage_limits: Vec<i32>) -> i32 {
        let mut usage_map: Vec<(i32, i32)> = Vec::new();

        for (i, val) in usage_limits.iter().enumerate() {
            usage_map.push((i as i32, *val));
        }

        usage_map.sort_by(|a, b| b.1.cmp(&a.1));
        println!("usage_map: {:?}", usage_map);

        let mut len: usize = 0;
        loop {
            match usage_map.len() {
                0 => return len as i32,
                l if l <= len => {
                    return len as i32;
                }
                _ => {}
            }

            let tmp = usage_map.clone();
            let mut removed: usize = 0;
            for (i, num) in (&tmp[0..len + 1]).iter().enumerate() {
                if num.1 == 1 {
                    usage_map.remove(i - removed);
                    removed += 1;
                } else {
                    usage_map[i].1 -= 1;
                }
            }

            len += 1;
        }
        return len as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::max_increasing_groups(vec![1, 6, 3, 5, 2, 8, 11, 9]),
            8,
            "Test 0"
        );
        assert_eq!(Solution::max_increasing_groups(vec![2, 1, 2]), 2, "Test 1");
        assert_eq!(Solution::max_increasing_groups(vec![1, 2, 5]), 3, "Test 2");
        assert_eq!(Solution::max_increasing_groups(vec![1, 1]), 1, "Test 3");
    }
}
