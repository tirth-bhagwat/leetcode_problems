// https://leetcode.com/problems/assign-cookies/

struct Solution {}

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();

        let mut s = s.iter();

        g.iter()
            .filter_map(|x| {
                while let Some(cookie) = s.next() {
                    if cookie >= x {
                        return Some(1);
                    }
                }
                None
            })
            .count() as i32

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1,
            "Test 1"
        );

        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2,
            "Test 2"
        );

        assert_eq!(
            Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8]),
            2,
            "Test 3"
        );
    }
}
