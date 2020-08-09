pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        use std::iter;
        let a_chars: Vec<u32> = a.chars().map(|n| n.to_digit(10).unwrap()).collect();
        let b_chars: Vec<u32> = b.chars().map(|n| n.to_digit(10).unwrap()).collect();

        let zipped = if a_chars.len() >= b_chars.len() {
            a_chars
                .into_iter()
                .rev()
                .zip(b_chars.into_iter().rev().chain(iter::repeat(0)))
        } else {
            b_chars
                .into_iter()
                .rev()
                .zip(a_chars.into_iter().rev().chain(iter::repeat(0)))
        };

        let mut carry = 0;
        let mut result: Vec<u32> = Vec::new();

        for (a, b) in zipped {
            let added = carry + a + b;
            if added >= 2 {
                carry = 1;
                result.push(added - 2);
            } else {
                carry = 0;
                result.push(added);
            }
        }

        if carry == 1 {
            result.push(1);
        }

        result
            .iter()
            .rev()
            .fold(String::new(), |acc, n| acc + &n.to_string())
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::add_binary(
                "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(),
                "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()),
                "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000".to_string()
        );
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::add_binary("1".to_string(), "111".to_string()),
            "1000".to_string()
        );
    }
}
