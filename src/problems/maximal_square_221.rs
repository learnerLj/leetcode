pub struct Solution;

impl Solution {
   pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
    dp[0] = matrix[0].iter().map(|&c| c.to_digit(10).unwrap() as i32).collect();

    let mut max_res: i32 = *dp[0].iter().max().unwrap();
    for i in 1..m {
        for j in 0..n {
            dp[i][j] = if matrix[i][j] == '1' {
                let left_up = if j > 0 { dp[i - 1][j - 1] } else { 0 };
                let left = if j > 0 { dp[i][j - 1] } else { 0 };
                let up = dp[i - 1][j];
                left_up.min(left).min(up) + 1
            } else {
                0
            };
            max_res = max_res.max(dp[i][j] * dp[i][j]);
        }
    }
    max_res
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximal_square_mixed_ones_and_zeros() {
        let matrix = vec![
            vec!['1', '0'],
            vec!['0', '0']
        ];
        assert_eq!(Solution::maximal_square(matrix), 1);
    }
    #[test]
    fn maximal_square_all_zeros() {
        let matrix = vec![
            vec!['0', '0', '0'],
            vec!['0', '0', '0'],
            vec!['0', '0', '0']
        ];
        assert_eq!(Solution::maximal_square(matrix), 0);
    }

    #[test]
    fn maximal_square_all_ones() {
        let matrix = vec![
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
            vec!['1', '1', '1']
        ];
        assert_eq!(Solution::maximal_square(matrix), 9);
    }

    #[test]
    fn maximal_square_mixed_values() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ];
        assert_eq!(Solution::maximal_square(matrix), 4);
    }

    #[test]
    fn maximal_square_single_row() {
        let matrix = vec![vec!['1', '0', '1', '0', '0']];
        assert_eq!(Solution::maximal_square(matrix), 1);
    }

    #[test]
    fn maximal_square_single_column() {
        let matrix = vec![vec!['1'], vec!['0'], vec!['1'], vec!['0'], vec!['0']];
        assert_eq!(Solution::maximal_square(matrix), 1);
    }
}