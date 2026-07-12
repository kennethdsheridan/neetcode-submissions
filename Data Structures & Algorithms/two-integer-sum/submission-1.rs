impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {   
        let mut map = std::collections::HashMap::new();

        for (current_index, &current_value) in nums.iter().enumerate() {
            let diff = target - current_value;

            if let Some(&matching_index) = map.get(&diff) {
                return vec![matching_index as i32, current_index as i32];
            }
            map.insert(current_value, current_index);
        }
        return vec![]


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_two_sum() {
        let input = vec![2, 3, 5, 6, 7];
        let target = 9;
        let result = Solution::two_sum(input, target);
        assert_eq!(result, vec![]);
    }
}