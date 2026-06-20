use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // if the lengths of both strings are different, immediately return false
        if s.len() != t.len() {
            return false
        }
        let mut s_count: HashMap<char, i32> = HashMap::new();
        let mut t_count: HashMap<char, i32> = HashMap::new();
        let x: Vec<(char, char)> = s.chars().zip(t.chars()).collect();
        for (char_s, char_t) in x {
            *s_count.entry(char_s).or_insert(0) += 1;
            *t_count.entry(char_t).or_insert(0) += 1;
        }
        s_count == t_count
        // if s_count.len() != t_count.len() {
        //     return false
        // }
        // for (k,v) in &s_count {
        //     match t_count.get(k) {
        //         Some(y) if y == v => continue,
        //         _ => return false,
        //     }
        // }
        // return true
    }
}
