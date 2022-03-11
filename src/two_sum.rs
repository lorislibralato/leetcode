
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n1) in nums.iter().enumerate() {
        for (j, n2) in nums.iter().skip(i+1).enumerate() {
            if n1 + n2 == target{
                return vec![i as i32, (j+i+1) as i32];
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_check(){
        assert_eq!(two_sum(vec![], 0), vec![]);
    }

    #[test]
    fn check() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 3], 6), vec![0, 2]);
        assert_eq!(
            two_sum(vec![28, 1, 24, 2, 7, 18, 10, 27, 11, 9, 8, 3, 12, 5, 29, 30], 59),
            vec![14, 15]
        );
    }
}