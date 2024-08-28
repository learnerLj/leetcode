pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // nums = [1,2,3], we should count from the end to the start
        //the last two elements are 2 and 3, 2 < 3, so we should swap them

        // nums = [3,2,1], the last two elements are 2 and 1, 2 > 1, so [2,1] is the largest
        // permutation. We should find the element right before them, which is 3,
        // the largest element in 1,2,3. Then continue to consider the next elements from the end.
        // If the element is the first element,  we should reverse the array

        // nums = [1,3,2], the last two elements are 3 and 2, 3 > 2, so [3,2] is the largest permutation
        // when considering the last three elements, 1,3,2, 1 < 2 and 2, so we should swap the
        // minimum element that larger than 1, which is 2, with 1, and sort the rest elements to get
        // the smallest permutation

        let n = nums.len();
        let mut i = n - 1;
        while i > 0 {
            i -= 1;
            let current = nums[i];
            let rest = &nums[i + 1..];
            // the rest elements are already sorted in descending order, if not, we have found the
            // next permutation by swapping the larger element behind the smaller one.

            if let Some((j, _)) = rest.iter().enumerate().find(
                |&(j, &num)|
                    (num > current) && if j != rest.len() - 1 { rest[j + 1] <= current } else { true }) {
                nums.swap(i, i + j + 1); // swap the current element with the larger one
                nums[i + 1..].sort(); // sort the rest elements to get the smallest permutation
                return;
            }
        }
        // if we reach here, it means the array is already the largest permutation, we should reverse it
        nums.reverse();
    }

    pub fn next_permutation2(nums: &mut Vec<i32>) {
        let l = nums.len();
        if !Self::find_and_swap(nums, l) {
            nums.reverse();
        }
    }

    fn find_and_swap(nums: &mut [i32], i: usize) -> bool {
        if i == 0 {
            return false;
        }

        let current = nums[i - 1];

        if let Some((j, _)) = nums[i..].iter().enumerate().find(|&(j, &num)| {
            num > current && (j == nums[i..].len() - 1 || nums[i..][j + 1] <= current)
        }) {
            nums.swap(i - 1, i + j);
            nums[i..].sort();
            return true;
        }

        Self::find_and_swap(nums, i - 1)
    }

    pub fn next_permutation3(nums: &mut Vec<i32>) {
        let swap_indices = (0..nums.len() - 1)
            .rev()
            .find_map(|i| {
                let current = nums[i];
                let rest = &nums[i + 1..];
                rest.iter()
                    .enumerate()
                    .find(|&(j, &num)| {
                        num > current && (j == rest.len() - 1 || rest[j + 1] <= current)
                    })
                    .map(|(j, _)| (i, i + j + 1))
            });

        if let Some((i, j)) = swap_indices {
            nums.swap(i, j);
            nums[i + 1..].sort();
        } else {
            nums.reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_permutation() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
        Solution::next_permutation2(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);
        Solution::next_permutation3(&mut nums);
        assert_eq!(nums, vec![2, 3, 1]);

        let mut nums = vec![1, 3, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);
        Solution::next_permutation2(&mut nums);
        assert_eq!(nums, vec![2, 3, 1]);
        Solution::next_permutation3(&mut nums);
        assert_eq!(nums, vec![3, 1, 2]);

        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
        Solution::next_permutation2(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
        Solution::next_permutation3(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);

        let mut nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
        Solution::next_permutation2(&mut nums);
        assert_eq!(nums, vec![1]);
        Solution::next_permutation3(&mut nums);
        assert_eq!(nums, vec![1]);

        let mut nums = vec![1, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1]);
        Solution::next_permutation2(&mut nums);
        assert_eq!(nums, vec![1, 2]);
        Solution::next_permutation3(&mut nums);
        assert_eq!(nums, vec![2, 1]);
    }
}