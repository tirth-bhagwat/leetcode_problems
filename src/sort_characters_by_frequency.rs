// https://leetcode.com/problems/sort-characters-by-frequency/

struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freq = [0_usize; (b'z' - b'0' + 1) as usize];

        for i in s.bytes() {
            freq[(i - b'0') as usize] = freq[(i - b'0') as usize] + 1;
        }

        let mut new_freq: Vec<(u8, usize)> = freq
            .iter()
            .enumerate()
            .filter_map(|(i, f)| if *f <= 0 { None } else { Some((i as u8, *f)) })
            .collect();

        new_freq.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        println!("{:?}", new_freq);

        new_freq
            .iter()
            .map(|(c, f)| vec![(*c + b'0') as char; *f].iter().collect::<String>())
            .collect()
    }
}
