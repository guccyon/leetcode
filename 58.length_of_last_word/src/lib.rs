pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.split_whitespace().rev().collect();
        match words.first() {
            None => 0,
            Some(word) => word.len() as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_last_word("hello world".to_string()), 5);
    }
}
