// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/

struct Solution {}

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let bank = bank
            .into_iter()
            .map(|x| x.as_bytes().iter().filter(|a| a == &&b'1').count())
            .filter(|x| x != &0)
            .collect::<Vec<usize>>();

        if bank.len() < 2 {
            return 0;
        }

        let mut res = 0;
        let mut prev = bank[0];

        for i in bank.iter().skip(1) {
            res += prev * i;
            prev = *i;
        }

        res as i32
    }
}
