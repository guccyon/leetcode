struct Solution;

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        match num {
            0 => false,
            1 => true,
            _ => {
                let log = (num as f64).log(4.0);
                log.round() == log
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_power_of_four(0), false);
        assert_eq!(Solution::is_power_of_four(1), true);
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(5), false);
    }
}
