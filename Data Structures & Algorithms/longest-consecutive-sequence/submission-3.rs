use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        /*
            convert nums to hash set
            for each num in hash set, if num - 1 does not exist, then this 
            is a potential starter number in a sequence of consecutive numbers
            
            now that we have the set of starter numbers, we create a hashmap where
            the key is that starter number, and we keep checking if the consecutive
            number is in the hash set and we store as a value the length, which is i32

            at the end, we take the max(hashmap values)

            conversion: O(n)
            num-1 check in HashSet: O(1)
            iterate each starter number O(n) worst case if each number is a starter number
            HashSet check for n+1: O(1)
            product of the two above: O(n)
            incrementing the value for each hash map key: O(1)

            final space and time complexity: O(n)
        */
        // do a check here on length 0 array
        if nums.len() == 0 {
            return 0;
        }
        let nums_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut result_map: HashMap<i32, i32> = HashMap::new();
        for n in &nums {
            // Check if n-1 exists in nums_set
            // deref &n
            let prev_num: i32 = (*n - 1) as i32;
            if !nums_set.contains(&prev_num) {
                // n is a starter number, store it in result_map
                result_map.entry(*n).or_insert(0);
            }
        }
        // iterate through result_map and increment val if n+1 exists in nums_set
        for (k, v) in &mut result_map {
            let mut next_consecutive_num: i32 = k+1;
            loop {
                *v += 1;
                if !nums_set.contains(&next_consecutive_num) {
                    break;
                }
                next_consecutive_num += 1;
            }
        }
        // println!("{:?}", result_map);
        let max_val = result_map.values().max();
        match max_val {
            Some(v) => return *v,
            None => return 0,
        }
    }
}
