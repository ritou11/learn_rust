// 42 Trapping Rain Water
use crate::Solution;

impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = height.len() - 1;
        let mut res: i32 = 0;
        while start + 1 < end {
            while start < end && height[start] == 0 {
                start += 1;
            }
            while start < end && height[end] == 0 {
                end -= 1;
            }
            if start == end {
                break;
            }
            let mut base = end - start + 1;
            for i in start..=end {
                if height[i] > 0 {
                    height[i] -= 1;
                    base -= 1;
                }
            }
            res += base as i32;
        }
        res
    }
}
