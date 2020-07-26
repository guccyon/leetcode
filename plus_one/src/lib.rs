pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = digits.into_iter().rev().fold(vec![1], |mut ac, e| {
            let value = ac.pop().unwrap();
            if value + e >= 10 {
                ac.push(0);
                ac.push(1);
            } else {
                ac.push(value + e);
                ac.push(0);
            }
            ac
        });

        if result.last().unwrap() == &0 {
            result.pop();
        }

        result.into_iter().rev().collect()
    }

    pub fn plus_one_fail(digits: Vec<i32>) -> Vec<i32> {
        let joined: String = digits
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("");

        let mut num: u64 = joined.parse().unwrap();
        num += 1;

        num.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test1() {
        let p = vec![1, 2, 3];
        assert_eq!(Solution::plus_one(p), vec![1, 2, 4]);
    }

    #[test]
    fn test2() {
        let p = vec![4, 3, 2, 1];
        assert_eq!(Solution::plus_one(p), vec![4, 3, 2, 2]);
    }

    #[test]
    fn test3() {
        let p = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(Solution::plus_one(p), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1]);
    }

    #[test]
    fn test4() {
        let p = vec![
            7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4,
            7, 0, 1, 1, 1, 7, 4, 0, 0, 6,
        ];
        assert_eq!(
            Solution::plus_one(p),
            vec![
                7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9,
                4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 7
            ]
        );
    }

    #[test]
    fn test5() {
        let p = vec![1, 9, 9];
        assert_eq!(Solution::plus_one(p), vec![2, 0, 0]);
    }
}
