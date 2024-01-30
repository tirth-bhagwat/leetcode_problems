// https://leetcode.com/problems/evaluate-reverse-polish-notation/

struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut s: Vec<i32> = vec![];
        let tokens: Vec<&[u8]> = tokens.iter().map(|x| x.as_bytes()).collect();
        for t in tokens {
            println!("{:?}", &s);
            if t.len() == 1 {
                match t[0] {
                    b'+' => {
                        let a = s.pop().unwrap();
                        let b = s.pop().unwrap();
                        s.push(b + a);
                    }
                    b'-' => {
                        let a = s.pop().unwrap();
                        let b = s.pop().unwrap();
                        s.push(b - a);
                    }
                    b'*' => {
                        let a = s.pop().unwrap();
                        let b = s.pop().unwrap();
                        s.push(b * a);
                    }
                    b'/' => {
                        let a = s.pop().unwrap();
                        let b = s.pop().unwrap();
                        s.push(b / a);
                    }
                    _ => {
                        s.push(Solution::to_num(t));
                    }
                };
            } else {
                s.push(Solution::to_num(t));
            }
            println!("  {:?}", &s);
        }

        s[0]
    }

    pub fn to_num(num: &[u8]) -> i32 {
        let mut res: i32 = 0;
        for i in num {
            if *i != b'-' {
                res *= 10;
                res += (i - b'0') as i32;
            }
        }

        if num[0] == b'-' {
            res *= -1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(
        //     Solution::eval_rpn(
        //         vec!["2", "1", "+", "3", "*"]
        //             .iter()
        //             .map(|x| x.to_string())
        //             .collect()
        //     ),
        //     9
        // );

        // ["4","13","5","/","+"]

        // assert_eq!(
        //     Solution::eval_rpn(
        //         vec!["4", "13", "5", "/", "+"]
        //             .iter()
        //             .map(|x| x.to_string())
        //             .collect()
        //     ),
        //     6
        // );
        // ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]

        assert_eq!(
            Solution::eval_rpn(
                vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            22
        );
    }
}
