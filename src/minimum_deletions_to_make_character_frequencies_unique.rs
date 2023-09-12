// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/

struct Solution {}

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counts = vec![0; 26];
        for c in s.chars() {
            counts[c as usize - 97] += 1;
        }
        counts = counts.iter().filter(|x| **x != 0).map(|x| *x).collect();
        counts.sort();

        let (mut i, mut j) = (0, 0);
        let mut unused = Vec::<usize>::new();
        let mut expected = 1;
        let mut deletions = 0;
        let counts_len = counts.len();
        while j < counts_len {
            if j + 1 < counts_len {
                if counts[j + 1] == counts[j] {
                    j += 1;
                    continue;
                }
            }

            if counts[i] != expected {
                for k in expected..counts[i] {
                    unused.push(k);
                }
                expected = counts[i] + 1;
            } else {
                expected += 1
            }
            if i != j {
                deletions += Solution::get_count(&counts[i..j], &mut unused)
            }
            j += 1;
            i = j;
        }

        return deletions as i32;
    }

    pub fn get_count(arr: &[usize], unused: &mut Vec<usize>) -> usize {
        let mut res = 0;
        let mut i = 0;
        let arr_len = arr.len();
        while !unused.is_empty() && i < arr_len {
            res += arr[i] - unused.pop().unwrap();

            i += 1;
        }
        if i < arr_len {
            res += arr[i..].iter().sum::<usize>();
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::min_deletions("aaaaabbbbbbcc".to_string()),
            0,
            "test 1"
        );
        assert_eq!(Solution::min_deletions("aaabbbcc".to_string()), 2, "test 2");
        assert_eq!(Solution::min_deletions("aab".to_string()), 0, "test 3");
        assert_eq!(Solution::min_deletions("ceabaacb".to_string()), 2, "test 4");
        assert_eq!(
            Solution::min_deletions("accdcdadddbaadbc".to_string()),
            1,
            "test 5"
        );
    }
}
