pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            match (c, stack.last()) {
                (')', Some('(')) => {
                    stack.pop();
                }
                ('}', Some('{')) => {
                    stack.pop();
                }
                (']', Some('[')) => {
                    stack.pop();
                }
                _ => stack.push(c),
            }
        }
        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
    }
    #[test]
    fn test2() {
        assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
    }
    #[test]
    fn test3() {
        assert_eq!(false, Solution::is_valid("(]".to_string()));
    }
    #[test]
    fn test4() {
        assert_eq!(false, Solution::is_valid("([)]".to_string()));
    }
    #[test]
    fn test5() {
        assert_eq!(true, Solution::is_valid("{[]}".to_string()));
    }
    #[test]
    fn test6() {
        assert_eq!(false, Solution::is_valid("{[{{}]}".to_string()));
    }
    #[test]
    fn test7() {
        assert_eq!(false, Solution::is_valid("]".to_string()));
    }
}
