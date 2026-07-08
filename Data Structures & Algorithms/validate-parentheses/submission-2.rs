use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut mappings: HashMap<char, char> = HashMap::new();
        mappings.insert('(', ')');
        mappings.insert('[', ']');
        mappings.insert('{', '}');
        for c in s.chars() {
            if mappings.contains_key(&c) {
                stack.push(c);
            } else {
                let Some(last) = stack.pop() else {return false;};
                match mappings.get(&last) {
                    Some(v) => {
                        if *v != c { return false; }
                    },
                    None => {return false;}
                }
            }
        }
        return stack.is_empty();
    }
}
