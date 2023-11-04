// https://leetcode.com/problems/build-an-array-with-stack-operations/

struct Solution {}

impl Solution {
    pub fn build_array(mut target: Vec<i32>, n: i32) -> Vec<String> {
        let mut j = 0;
        let mut targ_len = target.len();

        for i in 1..=n {
            if j == targ_len {
                break;
            }
            if target[j] == i {
                target[j] = 0;
                j += 1;
            } else {
                target.insert(j, 1);
                target.insert(j, 0);
                j = j + 2;
                targ_len+=2;
            }
        }


        target
            .iter()
            .map(|x| match x {
                0 => "Push".to_string(),
                1 => "Pop".to_string(),
                _ => panic!("Error"),
            })
            .collect::<Vec<String>>()
    }
}

// let mut j = 0;
// let mut res = vec![];
// let targ_len = target.len();
//
// for i in 1..=n {
// if j == targ_len {
// break;
// }
// if target[j] == i {
// res.push("Push");
// j += 1;
// } else {
// res.push("Push");
// res.push("Pop");
// }
// }

// res.iter()
// .map(|x| String::from(*x))
// .collect::<Vec<String>>()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec!["Push", "Push", "Pop", "Push"]
                .iter()
                .map(|x| String::from(*x))
                .collect::<Vec<String>>(),
            "Test 1"
        );

        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec!["Push", "Push", "Push"]
                .iter()
                .map(|x| String::from(*x))
                .collect::<Vec<String>>(),
            "Test 2"
        );

        assert_eq!(
            Solution::build_array(vec![1, 2], 4),
            vec!["Push", "Push"]
                .iter()
                .map(|x| String::from(*x))
                .collect::<Vec<String>>(),
            "Test 3"
        );
        assert_eq!(
            Solution::build_array(vec![2, 3, 4], 4),
            vec!["Push", "Push"]
                .iter()
                .map(|x| String::from(*x))
                .collect::<Vec<String>>(),
            "Test 3"
        );
    }
}
