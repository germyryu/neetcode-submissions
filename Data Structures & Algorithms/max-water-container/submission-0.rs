impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        /*
            two pointers, one at the start, one at the end
            to calculate the amonut of water a container can store:
            - it's (right-left) * min(heights[right], heights[left])
            store the maximum value of volume in a variable
            if heights[left] <= heights[right], then we increment left
            if heights[left] > heights[right], then we decrement right
        */
        let mut left: usize = 0;
        let mut right: usize = heights.len()-1;
        let mut max_volume = 0;
        while left < right {
            let volume = (heights[left].min(heights[right])) * (right as i32 - left as i32);
            // println!("{} {} {}", left, right, volume);
            max_volume = max_volume.max(volume);
            if heights[left] <= heights[right] {
                left += 1;
                continue;
            }
            if heights[left] > heights[right] {
                right -= 1;
                continue;
            }
        }
        return max_volume;
    }
}
