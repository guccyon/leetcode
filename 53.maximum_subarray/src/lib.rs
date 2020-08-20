struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut maxSum = i32::MIN;
        let mut curSum = 0;
        for i in nums {
            if (i < curSum + i) {
                curSum += i;
            } else {
                curSum = i;
            }
            maxSum = if curSum > maxSum { curSum } else { maxSum }
        }
        maxSum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_sub_array(vec![5, -4, 1, 2, 3]), 7);
    }
}
