/*
length of s has to be greater than or equal to t
window should be of size >= t.len() in order to have all of 
t's characters present in the substring

we create an array and +1 each character for t
we create another array and +1 each character for s[left..right]

left starts at 0
right starts at t.len()

for each substring, we check if it has all the characters in t
- if it doesn't we do right + 1 to increase the window size
- if it does, we have a valid substring and we should have another
loop here to do left + 1 and keep increasing
the window size to try and find a valid shorter substring

*/
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut result = String::new();
        let mut shortest_substring_length = 100000;
        if s.len() < t.len() {
            return result;
        }

        let mut t_count = [0; 128];
        for b in t.bytes() {
            t_count[b as usize] += 1;
        }
        let mut left = 0;
        let mut right = t.len();

        while right <= s.len() {
            // create a slice of s[left..right]
            let mut sub = &s[left..right];
            let mut s_count = [0; 128];
            for b in sub.bytes() {
                s_count[b as usize] += 1;
            }
            let is_subset = t_count.iter().zip(s_count.iter()).all(|(&x, &y)| x <= y);
            // if it is a valid subset and substring, we should look at making this substring smaller via a loop that incr. left
            // if not, then we increase the sliding window size by incr. right
            if !is_subset {
                right += 1;                
                continue;
            } else {
                if sub.len() < shortest_substring_length {
                    result = sub.to_string();
                    shortest_substring_length = shortest_substring_length.min(sub.len());
                }
                left += 1;
                continue;
            }
        }
        return result;
    }
}
