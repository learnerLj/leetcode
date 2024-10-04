pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let s = s.as_bytes();
        let mut dp = vec![vec![false; n]; n];
        let mut start = 0;
        let mut max_len = 0;

        // i is the start index, j is the gap starting from i
        for j in 0..n {
            for i in 0..(n - j) {
                dp[i][i + j] = match j {
                    0 => true,
                    1 => s[i] == s[i + 1],
                    _ => dp[i + 1][i + j - 1] && s[i] == s[i + j],
                };
                if dp[i][i + j] && j + 1 > max_len {
                    start = i;
                    max_len = j + 1;
                }
            }
        }
        String::from_utf8(s[start..start + max_len].to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_palindrome_single_character() {
        let s = String::from("a");
        assert_eq!(Solution::longest_palindrome(s), "a");
    }

    #[test]
    fn longest_palindrome_two_identical_characters() {
        let s = String::from("bb");
        assert_eq!(Solution::longest_palindrome(s), "bb");
    }

    #[test]
    fn longest_palindrome_mixed_characters() {
        let s = String::from("babad");
        let result = Solution::longest_palindrome(s);
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn longest_palindrome_all_identical_characters() {
        let s = String::from("aaaa");
        assert_eq!(Solution::longest_palindrome(s), "aaaa");
    }

    #[test]
    fn longest_palindrome_no_palindrome() {
        let s = String::from("abc");
        let result = Solution::longest_palindrome(s);
        assert!(result == "a" || result == "b" || result == "c");
    }

    #[test]
    fn longest_palindrome_empty_string() {
        let s = String::from("");
        assert_eq!(Solution::longest_palindrome(s), "");
    }

    #[test]
    fn longest_palindrome_palindrome_in_middle() {
        let s = String::from("abacdfgdcaba");
        assert_eq!(Solution::longest_palindrome(s), "aba");
    }
}