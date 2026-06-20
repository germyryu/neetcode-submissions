use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut s: HashSet<i32> = HashSet::new();
        for num in nums {
            if s.contains(&num) {
                return true
            } else {
                s.insert(num);
            }
        }
        return false
    }
}
