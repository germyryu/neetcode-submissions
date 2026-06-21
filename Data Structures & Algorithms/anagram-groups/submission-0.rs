use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // This maps alphabet frequency array -> vector of original strings
        let mut anagrams: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut count = [0; 26];
            for b in s.bytes() {
                let idx = (b - b'a') as usize;
                count[idx] += 1;
            }
            anagrams.entry(count).or_insert_with(Vec::new).push(s);
        }
        anagrams.into_values().collect()
    }
}
