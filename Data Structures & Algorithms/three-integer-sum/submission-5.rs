impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        /*
            basically two sum, but we fix one number and run two sum on the other numbers
            first need to sort the existing nums array so that we can set ourselves up for 2sum with pointers
            store a result vector that we push to everytime we find a triplet
        */
        let mut nums = nums; // this is a move, not copy, same heap buffer
        nums.sort(); // this sorts in-place so O(1) memory
        let mut result_vector: Vec<Vec<i32>> = Vec::new();
        // we can just increment i each time and ensure no duplicates are found as long as left < right
        for (i, x) in nums.iter().enumerate() {
            if nums[i] > 0 {
                break;
            }
            // perform two sum here on rest
            // target number should be (-x)
            if i > 0 && *x == nums[i-1] {
                continue;
            }
            let mut left = i+1;
            let mut right = nums.len() - 1;
            // println!("{}, {}", left, right);
            let target = -x;
            while left < right {
                if nums[left] + nums[right] > target {
                    right -= 1;
                    continue;
                }
                if nums[left] + nums[right] < target {
                    left += 1;
                    continue;
                }
                if nums[left] + nums[right] == target {
                    // push all three numbers as a vector to the result vec
                    let triplet = vec![*x, nums[left as usize], nums[right as usize]];
                    result_vector.push(triplet);
                    left += 1;
                    right -= 1;
                    // move left skip any duplicate values
                    while left < right && nums[left] == nums[left-1] {
                        left += 1;
                    }
                }
            }
        }
        return result_vector;
    }
}
