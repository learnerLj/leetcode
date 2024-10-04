pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 0..s.len() {
            if dp[i] {
                for w in &word_dict {
                    if s[i..].starts_with(w) {
                        dp[i + w.len()] = true;
                    }
                }
            }
        }
        dp.last().unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn word_break_single_word_match() {
        assert!(Solution::word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));
    }

    #[test]
    fn word_break_single_word_no_match() {
        assert!(!Solution::word_break("leetcode".to_string(), vec!["leet".to_string(), "cod".to_string()]));
    }

    #[test]
    fn word_break_multiple_words_match() {
        assert!(Solution::word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]));
    }

    #[test]
    fn word_break_multiple_words_no_match() {
        assert!(!Solution::word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]));
    }

    #[test]
    fn word_break_empty_string() {
        assert_eq!(Solution::word_break("".to_string(), vec!["leet".to_string(), "code".to_string()]), true);
    }

    #[test]
    fn word_break_no_words_in_dict() {
        assert!(!Solution::word_break("leetcode".to_string(), vec![]));
    }

    #[test]
    fn word_break_repeated_words() {
        assert!(Solution::word_break("aaaaaaa".to_string(), vec!["aaaa".to_string(), "aaa".to_string()]));
    }
}