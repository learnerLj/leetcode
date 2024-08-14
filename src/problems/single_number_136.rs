use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(nums.len()/2);
        nums.iter().for_each(|&num| {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        });
        map.into_iter().find(|(_, count)| *count == 1).unwrap().0
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_paths() {
        let result = Solution::single_number(vec![2, 2, 1]);
        assert_eq!(result, 1);
        
        let result = Solution::single_number(vec![4, 1, 2, 1, 2]);
        assert_eq!(result, 4);
    }
}