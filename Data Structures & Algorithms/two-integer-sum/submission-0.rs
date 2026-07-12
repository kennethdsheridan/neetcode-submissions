impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {   
        let mut map = std::collections::HashMap::new();

        for (i , &n) in nums.iter().enumerate() {
            let diff = target - n;
            if let Some(&index) = map.get(&diff) {
                return vec![index as i32, i as i32];
            }
            map.insert(n, i);
        }
        vec![]


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