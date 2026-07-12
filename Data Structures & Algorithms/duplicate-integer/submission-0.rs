use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        
        for &num in &nums {
            if !seen.insert(num) {
                return true;
            }
        
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate() {
            let nums = vec![2, 3, 5, 5, 6, 6];
            let result = Solution::has_duplicate(nums);
            assert_eq!(result, true);
    }
}