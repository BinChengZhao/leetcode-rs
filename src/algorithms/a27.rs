///remove-element
///原地算法.
struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut len = nums.len();
        let mut index = 0;

        while index < len {
            if nums[index] == val {
                nums.swap_remove(index);
                len = len - 1;
                continue;
            }
            index = index + 1;
        }

        len as i32
    }
}
