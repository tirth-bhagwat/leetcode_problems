// https://leetcode.com/problems/path-crossing/

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut curr_x = 0;
        let mut curr_y = 0;
        let mut visited: BTreeSet<(i32, i32)> = BTreeSet::new();
        visited.insert((curr_x, curr_y));

        for c in path.chars() {
            match c {
                'N' => {
                    curr_x += 1;
                }
                'S' => {
                    curr_x -= 1;
                }
                'E' => {
                    curr_y += 1;
                }
                'W' => {
                    curr_y -= 1;
                }
                _ => unreachable!(),
            }
            println!("({},{})", curr_x, curr_y);
            dbg!(&visited);
            if visited.contains(&(curr_x, curr_y)) {
                return true;
            }

            visited.insert((curr_x, curr_y));
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_path_crossing(String::from("NES")),
            false,
            "Test 1"
        );
        assert_eq!(
            Solution::is_path_crossing(String::from("NESWW")),
            true,
            "Test 2"
        );
    }
}
