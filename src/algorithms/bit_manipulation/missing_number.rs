pub struct Solution268;
impl Solution268 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut miss = nums.len() as i32;

        for (index, value) in nums.into_iter().enumerate() {
            miss ^= index as i32 ^ value;
        }

        miss
    }
}
