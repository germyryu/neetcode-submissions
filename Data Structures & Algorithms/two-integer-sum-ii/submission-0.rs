impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len()-1;
        loop {
            if numbers[left] + numbers[right] > target {
                right -= 1;
                continue;
            }
            if numbers[left] + numbers[right] < target {
                left += 1;
                continue;
            }
            break;
        }
        let l = left as i32;
        let r = right as i32;
        return vec![l+1,r+1];
    }
}
