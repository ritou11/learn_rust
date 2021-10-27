// 75 Sort Colors
use crate::Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 { return; }
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut i = 0;
        while i <= r {
            match nums[i] {
                0 => {
                    nums.swap(i, l);
                    i += 1;
                    l += 1;
                },
                2 => {
                    nums.swap(i, r);
                    if r == 0 { return; }
                    r -= 1;
                },
                _ => { i += 1; }
            }
        }
    }
}
