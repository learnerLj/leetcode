pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![vec![0; n]; n];
        dp[0][0] = triangle[0][0];
        for i in 1..n {
            for j in 0..=i {
                if j == 0 {
                    dp[i][j] = dp[i - 1][j] + triangle[i][j];
                } else if j == i {
                    dp[i][j] = dp[i - 1][j - 1] + triangle[i][j];
                } else {
                    dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]) + triangle[i][j];
                }
            }
        }
        dp[n - 1].iter().min().unwrap().to_owned()
    }
}

mod tests {
    use super::*;
    #[test]
    fn minimum_total_single_element() {
        let triangle = vec![vec![5]];
        assert_eq!(Solution::minimum_total(triangle), 5);
    }

    #[test]
    fn minimum_total_single_row() {
        let triangle = vec![vec![2, 3]];
        assert_eq!(Solution::minimum_total(triangle), 2);
    }

    #[test]
    fn minimum_total_two_rows() {
        let triangle = vec![vec![2], vec![3, 4]];
        assert_eq!(Solution::minimum_total(triangle), 5);
    }

    #[test]
    fn minimum_total_multiple_rows() {
        let triangle = vec![
            vec![2],
            vec![3, 4],
            vec![6, 5, 7],
            vec![4, 1, 8, 3]
        ];
        assert_eq!(Solution::minimum_total(triangle), 11);
    }

    #[test]
    fn minimum_total_large_values() {
        let triangle = vec![
            vec![1000],
            vec![2000, 3000],
            vec![4000, 5000, 6000]
        ];
        assert_eq!(Solution::minimum_total(triangle), 7000);
    }
}