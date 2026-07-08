/*
Keep a HashMap for s1
Maps character -> frequency of that character in s1

Iterate through s2 using left and right pointers
Initial sliding window should be of size: s1.len()
Keep moving the sliding window until we get that the count of the
characters in substring in s2 is the same as the HashMap we created

To save on space, we could initialize an array of size 26 for each letter of the alphabet
^^^
*/

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut s1_counts = [0; 26];
        let s1_bytes: Vec<u8> = s1.bytes().collect();
        for b1 in s1_bytes {
            s1_counts[(b1-b'a') as usize] += 1;
        }

        if s2.len() < s1.len() {
            return false;
        }

        let mut left = 0;
        let mut right = s1.len();
        while right <= s2.len() {
            let mut s2_substring = &s2[left..right];
            let mut s2_substring_counts = [0; 26];
            let s2_bytes = s2_substring.as_bytes();
            for &b in s2_bytes {
                s2_substring_counts[(b-b'a') as usize] += 1;
            }
            if s2_substring_counts == s1_counts {
                return true;
            }
            left += 1;
            right += 1;
        }
        false
    }
}
