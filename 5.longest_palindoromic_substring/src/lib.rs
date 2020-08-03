pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut max: usize = 0;
        let mut palindrome = &chars[..1];
        for (start, c) in chars.iter().enumerate() {
            let mut end = start + 1;
            while let Some(n) = Solution::find_index(&chars, c, end) {
                let slice = &chars[start..=n];
                if max < slice.len() && Solution::is_palindrome(slice) {
                    max = slice.len();
                    palindrome = slice;
                }
                end = n + 1;
            }
        }
        palindrome.iter().clone().collect::<String>()
    }

    pub fn is_palindrome(chars: &[char]) -> bool {
        for (i, c) in chars.into_iter().rev().enumerate() {
            if c != &chars[i] {
                return false;
            }
        }
        true
    }

    pub fn find_index(chars: &[char], ch: &char, offset: usize) -> Option<usize> {
        chars
            .iter()
            .skip(offset)
            .position(|c| c == ch)
            .map(|s| s + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }
    #[test]
    fn test3() {
        let p = "jhgtrclvzumufurdemsogfkpzcwgyepdwucnxrsubrxadnenhvjyglxnhowncsubvdtftccomjufwhjupcuuvelblcdnuchuppqpcujernplvmombpdttfjowcujvxknzbwmdedjydxvwykbbamfnsyzcozlixdgoliddoejurusnrcdbqkfdxsoxxzlhgyiprujvvwgqlzredkwahexewlnvqcwfyahjpeiucnhsdhnxtgizgpqphunlgikogmsffexaeftzhblpdxrxgsmeascmqngmwbotycbjmwrngemxpfakrwcdndanouyhnnrygvntrhcuxgvpgjafijlrewfhqrguwhdepwlxvrakyqgstoyruyzohlvvdhvqmzdsnbtlwctetwyrhhktkhhobsojiyuydknvtxmjewvssegrtmshxuvzcbrabntjqulxkjazrsgbpqnrsxqflvbvzywzetrmoydodrrhnhdzlajzvnkrcylkfmsdode".to_string();
        assert_eq!(Solution::longest_palindrome(p), "hhktkhh".to_string());
    }
    #[test]
    fn test4() {
        let p = "a".to_string();
        assert_eq!(Solution::longest_palindrome(p), "a".to_string());
    }
    #[test]
    fn test5() {
        let p = "".to_string();
        assert_eq!(Solution::longest_palindrome(p), "".to_string());
    }

    #[test]
    fn test_find1() {
        assert_eq!(
            Solution::find_index(&("cbcde".chars().collect::<Vec<char>>()), &'c', 1),
            Some(2)
        )
    }

    #[test]
    fn test_find2() {
        assert_eq!(
            Solution::find_index(&("cbcde".chars().collect::<Vec<char>>()), &'f', 2),
            None
        )
    }
}
