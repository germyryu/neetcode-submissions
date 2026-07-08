use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        // the number of replacements needed would be
        // the current size of the window - freq of most frequent character
        // we keep track of the most frequent character through a hashmap
        // key is the unique character, so lookups are O(1) with a space of O(m)
        // time complexity of the entire function would be O(n) since we iterate through the
        // length of the string in one pass
        // keep track of the maximum window size that we've seen so far
        // if the current size of the window  - most frequent character > k then we shrink window size
        // by moving the left pointer and also decrementing the freq of that character in the hashmap

        let mut freq : HashMap<char, i32> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        // given that the constraint is s.length >= 1, we can initialize the 
        // character with the max frequency to be the first character
        let mut max_freq_char: char = chars[0];
        let mut max_freq = 0;
        let mut left = 0;
        let mut right = 0;
        let mut max_substring = 0;

        // iterate until the end of the window reaches the end of the string
        while right < chars.len() {
            // check the character at the right pointer
            // check in the freq hashmap, this char is present
            // if it isn't add it in and give it the frequency of 1
            // if it is there, then increment it and use this incremented value
            // to check if it is greater than max_freq.
            // if it is, then we update max_freq_char and max_freq
            // then do a check to see if max_freq <= k
            // if so, we increase the right pointer
            // if not, we need to increase the left pointer
            let c = freq.entry(chars[right as usize]).or_insert(0);
            *c += 1;
            if *c > max_freq {
                max_freq = *c;
            }

            // let diff = right - left + 1 - max_freq as usize;
            while right - left + 1 - max_freq as usize > k as usize {
                *freq.entry(chars[left]).or_insert(0) -= 1;
                left += 1;
            }
            max_substring = max_substring.max(right - left + 1);
            right += 1;
        }
        return max_substring as i32;
    }
}
