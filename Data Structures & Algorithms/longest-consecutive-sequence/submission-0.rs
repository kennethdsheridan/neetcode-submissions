use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut longest = 0;

        for &num in &nums_set {
           if !nums_set.contains(&(num - 1)) {
            let mut length = 1;
            while nums_set.contains(&(num + length)) {
                length += 1;
            }
            longest = longest.max(length);
           } 
        }
        longest

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        let input = vec![2,20,4,10,3,4,5];
        let result =  Solution::longest_consecutive(input);

        assert_eq!(result, 4);
    }
}