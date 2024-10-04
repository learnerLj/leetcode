pub struct Solution;


// https://leetcode.cn/problems/distinct-subsequences/solutions/2880141/dong-tai-gui-hua-de-liang-chong-si-lu-by-u25b/
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();

        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        // 创建 dp 数组，dp[i][j] 表示 t[..i] 在 s[..j] 中出现的次数
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for j in 0..=n {
            dp[0][j] = 1;
        }

        for i in 1..=m {
            for j in 1..=n {
                if t[i - 1] == s[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1];
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_distinct() {
        let s1 = "bba".to_string();
        let t1 = "ba".to_string();
        let result = Solution::num_distinct(s1, t1);
        assert_eq!(result, 2);

        let s2 = "rabbbit".to_string();
        let t2 = "rabbit".to_string();
        let result = Solution::num_distinct(s2, t2);
        assert_eq!(result, 3);
    }
}