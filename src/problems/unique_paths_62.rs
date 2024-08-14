use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut cache: HashMap<(i32, i32), i32> = HashMap::new();
        Self::dp(m, n, &mut cache)
    }

    fn dp(m: i32, n: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        // 检查缓存中是否已有结果
        if let Some(&result) = cache.get(&(m, n)) {
            return result;
        }

        // 计算结果并缓存
        let result = match (m, n) {
            (1, _) => 1,
            (_, 1) => 1,
            _ => Self::dp(m - 1, n, cache) + Self::dp(m, n - 1, cache),
        };

        cache.insert((m, n), result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_paths() {
        let result = Solution::unique_paths(3, 2);
        assert_eq!(result, 3);
        
        let result = Solution::unique_paths(51, 9);
        assert_eq!(result, 1916797311);
    }
}