// https://leetcode.com/problems/remove-colored-pieces-if-both-neighbors-are-the-same-color/

struct Solution {}

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        if colors.len() <= 2 {
            return false;
        }

        let mut a = 0;
        let mut b = 0;
        let mut it = colors.chars();
        let mut x = it.next().unwrap();
        let mut y = it.next().unwrap();
        let mut z = it.next().unwrap();

        if x == y && y == z && z == 'A' {
            a += 1;
        } else if x == y && y == z && z == 'B' {
            b += 1;
        }

        while let Some(new) = it.next() {
            x = y;
            y = z;
            z = new;
            if x == y && y == z && z == 'A' {
                a += 1;
            } else if x == y && y == z && z == 'B' {
                b += 1;
            }
        }

        return a > b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::winner_of_game("AAABABB".to_string()),
            true,
            "Test 1"
        );
        assert_eq!(Solution::winner_of_game("AA".to_string()), false, "Test 2");
        assert_eq!(
            Solution::winner_of_game("ABBBBBBBAAA".to_string()),
            false,
            "Test 3"
        );
    }
}
