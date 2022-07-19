struct Solution {}

// start submission

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        todo!()
    }
}

// end submission

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_check() {
        assert_eq!(Solution::two_sum(vec![], 0), vec![]);
    }

    #[test]
    fn check() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 3], 6), vec![0, 2]);
        assert_eq!(
            Solution::two_sum(
                vec![28, 1, 24, 2, 7, 18, 10, 27, 11, 9, 8, 3, 12, 5, 29, 30],
                59
            ),
            vec![14, 15]
        );
    }
}
