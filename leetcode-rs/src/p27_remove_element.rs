// 75 Sort Colors
use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 { return 0; }
        if nums.len() == 1 { return if nums[0] == val {0} else {1}; }
        let mut r = nums.len() - 1;
        let mut i = 0;
        while i <= r {
            if nums[i] == val {
                nums.swap(i, r);
                if r == 0 { return 0; }
                r -= 1;
            } else {
                i += 1;
            }
        }
        i as i32
    }
}
