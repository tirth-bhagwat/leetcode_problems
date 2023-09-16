// https://leetcode.com/problems/find-the-town-judge/

struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut people = vec![0; n as usize];

        println!("people: {:?}", people);
        for i in &trust {
            people[i[1] as usize - 1] += 1;
            println!("people: {:?}", people);
        }

        if people.iter().filter(|x| **x == n as usize - 1).count() == 1 {
            let poss_judge = people.iter().position(|x| *x == n as usize - 1).unwrap();
            let mut flag = true;
            for i in &trust {
                println!("i: {:?}", i);
                if i[0] as usize - 1 == poss_judge {
                    flag = false;
                    break;
                }
            }
            if flag {
                return people.iter().position(|x| *x == n as usize - 1).unwrap() as i32 + 1;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2, "Test 1");
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]),
            3,
            "Test 2"
        );
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1,
            "Test 3"
        );
        // [[1,2],[2,3]]
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 2], vec![2, 3]]),
            -1,
            "Test 4"
        );
        // [[1,3],[1,4],[2,3],[2,4],[4,3]]
        assert_eq!(
            Solution::find_judge(
                4,
                vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
            ),
            3,
            "Test 5"
        )
    }
}
