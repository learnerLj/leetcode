pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.to_ascii_lowercase();

        let mut parts = s.split('e');
        let (first, second, third) = (parts.next(), parts.next(), parts.next());
        if third.is_some() {
            return false;
        }
        match (first, second) {
            (Some(first), Some(second)) => {
                (Self::is_decimal(first) || Self::is_integer(first))
                    && Self::is_exp(&format!("{}{}", 'e', second))
            }
            (Some(first), None) => Self::is_decimal(first) || Self::is_integer(first),
            _ => false,
        }
    }

    fn is_digits(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
    }

    fn is_integer(s: &str) -> bool {
        let mut s = s;
        if s.starts_with('+') || s.starts_with('-') {
            s = &s[1..];
        }
        Self::is_digits(s)
    }

    fn is_exp(s: &str) -> bool {
        (s.starts_with('e') || s.starts_with('E')) && Self::is_integer(&s[1..])
    }

    fn is_decimal(s: &str) -> bool {
        let mut s = s;
        if s.starts_with('+') || s.starts_with('-') {
            s = &s[1..];
        }

        let mut parts = s.split('.');
        match (parts.next(), parts.next(), parts.next()) {
            // (_, _, Some(_)) => false,
            (Some(v), Some(""),None) => Self::is_digits(v),
            (Some(""), Some(e),None) => Self::is_digits(e),
            (Some(v), Some(e),None) => Self::is_digits(v) && Self::is_digits(e),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn is_number() {
        let s1 = "0".to_string();
        let s2 = "0.1".to_string();
        let s3 = "e".to_string();
        let s4 = "0..".to_string();
        assert!(Solution::is_number(s1));
        assert!(Solution::is_number(s2));
        assert!(!Solution::is_number(s3));
        assert!(!Solution::is_number(s4));
    }
}
