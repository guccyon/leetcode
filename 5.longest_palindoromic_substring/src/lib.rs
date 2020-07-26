pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max:usize = 0;
        let mut palindrome = String::from("");
        for (i, _) in s.chars().enumerate() {
            for j in (i + 1)..=s.len() {
                let chars: Vec<char> = s.chars().collect();
                let slice = &chars[i..j];
                if Solution::is_palidrome(slice) && max < slice.len() {
                    max = slice.len();
                    palindrome = slice.iter().cloned().collect();
                }
            }
        }
        println!("{}", max);
        palindrome
    }

    pub fn is_palidrome(chars: &[char]) -> bool {
        let length = chars.len();
        for (i, c) in chars.into_iter().enumerate() {
            if c != &chars[length - i - 1] {
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
    }
    #[test]
    fn test3() {
        let p = "tembwxtvddsttolegryohdlhxhycymqybzfzcbkzdwcekzfapmpfyeivfoeeqoqdhylziqpnyzuzeeutpqpalbddlliuehvkhqevgjdkskvphidcjmpcmetzwqkzcnxjcjywhfzplntbkuddmbcovearburjqyirbladcrhfkfdfgsmyhdsfmmxmslwkymkgaguilxghmfgaldcogtfnbqakctqtqakupwrxkmbjpmzqngwldmaugzizgwmediyzxevspxdwruyzrmnhchtxlgtb".to_string();
        assert_eq!(Solution::longest_palindrome(p), "bb".to_string());
    }
}
