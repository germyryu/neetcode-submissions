use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // Sliding window problem
        // intialize a sliding window of size 1
        // keep a hashset of characters i've seen in this sliding window
        // increase the sliding window size by 1
        // move it to the right and for each new character, check to see if it's in the hashset
        // if it is, we keep that size and keep moving
        // if it isn't we add the character and increment the sliding window size again
        // at the end, we return the sliding window size

        let chars: Vec<char> = s.chars().collect();
        let mut seen: HashSet<char> = HashSet::new();

        if chars.len() == 0 {
            return 0;
        }
        if chars.len() == 1 {
            return 1;
        }

        let mut left = 0;
        let mut right = 0;
        let mut size = 0;
        while right < chars.len() {
            if !seen.contains(&chars[right]) {
                seen.insert(chars[right]);
                size = size.max(right - left);
                right += 1;
            }
            else {
                seen.remove(&(chars[left]));
                left += 1;
            }
        }
        return size as i32 + 1;
    }
}
