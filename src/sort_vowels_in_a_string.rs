// https://leetcode.com/problems/sort-vowels-in-a-string/

struct Solution {}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels: Vec<u8> = s
            .chars()
            .filter_map(|x| {
                if matches!(x, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U') {
                    return Some(x as u8);
                }
                None
            })
            .collect();
        vowels.sort();
        let mut sorted = vowels.into_iter();
        s.chars()
            .map(|x| {
                if matches!(x, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U') {
                    return sorted.next().unwrap() as char;
                }
                x
            })
            .collect()
    }
// }
//
// impl Solution {
    pub fn sort_vowels_2(mut s: String) -> String {
        let mut indices: Vec<usize> = vec![];
        let mut chars: Vec<u8> = vec![];
        let mut bytes = Vec::with_capacity(s.len());
        for (i,c) in s.bytes().enumerate() {
            if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u'
                || c == b'A' || c == b'E' || c == b'I' || c == b'O' || c == b'U' {
                indices.push(i);
                chars.push(c);
            }
            bytes.push(c);
        }
            chars.sort_unstable();
        for (indicee, c) in indices.into_iter().zip(chars.into_iter()) {
            bytes[indicee] = c;
        }
        String::from_utf8(bytes).unwrap()
    }
}


