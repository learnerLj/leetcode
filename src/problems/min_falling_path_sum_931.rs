pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut dp = vec![vec![0; n]; n];
        dp[0] = matrix[0].clone();
        for i in 1..n {
            for j in 0..n {
                let left = if j > 0 { dp[i - 1][j - 1] } else { i32::MAX };
                let right = if j < n - 1 { dp[i - 1][j + 1] } else { i32::MAX };
                let up = dp[i - 1][j];
                dp[i][j] = matrix[i][j] + left.min(right).min(up);
            }
        }
        dp[n - 1].iter().min().unwrap().to_owned()
    }
}

mod tests {
    use super::*;

    #[test]
    fn min_falling_path_sum_multiple_rows_and_columns() {
        let matrix = vec![
            vec![2, 1, 3],
            vec![6, 5, 4],
            vec![7, 8, 9]
        ];
        assert_eq!(Solution::min_falling_path_sum(matrix), 13);
    }

    #[test]
    fn min_falling_path_sum_large_values() {
        let matrix = vec![
            vec![100, 200],
            vec![300, 400]
        ];
        assert_eq!(Solution::min_falling_path_sum(matrix), 400);
    }
}