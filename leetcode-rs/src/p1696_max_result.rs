use crate::Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let mut res = nums[0];
        let mut d = BinaryHeap::new();
        d.push((nums[0], 0));
        for i in 1..n {
            while d.len() > 0 {
                match d.peek() {
                    Some(&(val, idx)) => {
                        if idx + (k as usize) < i {
                            d.pop();
                            continue;
                        }
                        res = nums[i] + val;
                        d.push((res, i));
                        break;
                    },
                    None => { break; }
                }
            }
        }
        res
    }
}
