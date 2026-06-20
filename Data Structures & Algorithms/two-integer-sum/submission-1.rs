use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // this is a map of the diff -> index
        let mut diff_store: HashMap<i32, i32> = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            let diff = target - x;
            if let Some (&v) = diff_store.get(&diff) {
                return vec![v, i as i32];
            } else {
                diff_store.insert(*x, i as i32);
            }
        }
        vec![]
    }
}
