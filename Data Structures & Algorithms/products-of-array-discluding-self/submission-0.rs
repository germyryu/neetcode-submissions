use std::collections::HashMap;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        /*
            Create map of index -> [product before this index, product after this index]
            Two for loops: one that iterates forwards, one that iterates backwards
            On the forward iteration, keep track of a rolling product
            For each i in map, multiply the values, push to vector, return.
        */
        let mut prefixes: HashMap<i32, i32> = HashMap::new();
        let mut suffixes: HashMap<i32, i32> = HashMap::new();
        let mut prefix_product = 1;
        for (i, num) in nums.iter().enumerate() {
            let idx = i as i32;
            let entry = prefixes.entry(idx).or_insert(prefix_product);
            prefix_product *= num;
        }
        let mut suffix_product = 1;
        for (i, num) in nums.iter().enumerate().rev() {
            let idx = i as i32;
            let entry = suffixes.entry(idx).or_insert(suffix_product);
            suffix_product *= num;
        }
        // Now walk through the map and multiply the two values
        // Insert into vector
        let mut result = vec![0; nums.len()];
        for (k,v) in prefixes {
            if let Some(x) = suffixes.get(&k) {
                let k1 = k as usize;
                result[k1] = v * x;
            }
        }
        return result;
    }
}
