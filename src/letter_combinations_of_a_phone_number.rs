// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let keys: Vec<Vec<char>> = vec![
            vec![],
            vec![],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let digits_bytes = digits.as_bytes();

        match digits_bytes.len() {
            0 => vec![],

            1 => keys[(digits_bytes[0] - b'0') as usize]
                .clone()
                .into_iter()
                .map(|x| x.to_string())
                .collect(),

            _ => {
                let ch = digits_bytes[0] as char;
                let mut res = vec![];

                let sing_ltrs: Vec<String> = keys[(digits_bytes[0] - b'0') as usize]
                    .clone()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect();

                for comb in Solution::letter_combinations(digits.get(1..).unwrap().to_string()) {
                    for s in &sing_ltrs {
                        let mut x = s.clone();
                        x.push_str(&comb);
                        res.push(x);
                    }
                }

                res
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            "Test 1"
        );

        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new(),
            "Test 2"
        );

        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"],
            "Test 3"
        );
    }
}
