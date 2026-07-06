impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        /*
        key observations:
        - water can only be trapped if and only if it is surrounded by two bars
        - the total volume of water that is trapped is depending on the smaller bar
        - it's the depth of the water is depending on the current index's height
        - if we want to calculate the total maximum water that can be trapped, then we need to iterate
        through each index and check to see how much water can be trapped surrounding that index
        - then do the same backwards and calculate the maximum by summing
        */

        let mut prefix: Vec<i32> = Vec::new();
        let mut suffix: Vec<i32> =  vec![0; height.len()];

        let mut max_prefix: i32 = 0;
        let mut max_suffix: i32 = 0;
        for v in height.iter() {
            max_prefix = max_prefix.max(*v);
            prefix.push(max_prefix);
        }
        for (j, v) in height.iter().enumerate().rev() {
            max_suffix = max_suffix.max(*v);
            suffix[j] = max_suffix;
        }
        // min(prefix[i], suffix[i]) - height[i]. width of each bar is 1
        let mut result: i32 = 0;
        for i in 0..height.len() {
            let l = prefix[i] as i32;
            let r = suffix[i] as i32;
            result += l.min(r) - height[i];
        }
        return result;
    }
}
