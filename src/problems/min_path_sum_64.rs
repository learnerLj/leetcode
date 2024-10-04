pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // a->b->c, if a->c is the shortest path, then a->b, b->c are also the shortest path
        // so we can iterate from top left to bottom right, and update the value(a->b, b->c)
        // of each cell b.

        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![i32::MAX; n]; m];

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    dp[i][j] = grid[i][j];
                } else {
                    let left = if j > 0 { dp[i][j - 1] } else { i32::MAX };
                    let up = if i > 0 { dp[i - 1][j] } else { i32::MAX };
                    dp[i][j] = grid[i][j] + left.min(up);
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

mod tests {
    use super::*;

    #[test]
    fn min_path_sum_single_element() {
        let grid = vec![vec![5]];
        assert_eq!(Solution::min_path_sum(grid), 5);
    }

    #[test]
    fn min_path_sum_single_row() {
        let grid = vec![vec![1, 2, 3]];
        assert_eq!(Solution::min_path_sum(grid), 6);
    }

    #[test]
    fn min_path_sum_single_column() {
        let grid = vec![vec![1], vec![2], vec![3]];
        assert_eq!(Solution::min_path_sum(grid), 6);
    }

    #[test]
    fn min_path_sum_multiple_rows_and_columns() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1]
        ];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }

    #[test]
    fn min_path_sum_large_values() {
        let grid = vec![
            vec![100, 200],
            vec![300, 400]
        ];
        assert_eq!(Solution::min_path_sum(grid), 700);
    }
}