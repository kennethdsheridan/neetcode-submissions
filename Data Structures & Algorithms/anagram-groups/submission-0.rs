impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: HashMap<String, Vec<String>> = std::collections::HashMap::new();

        for s in &strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            let mut s_sorted: String = chars.into_iter().collect();
            
            res.entry(s_sorted).or_default().push(s.clone());

        }
        res.into_values().collect()
    }
}
