// 41 First Missing Positive
use crate::Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut marks: Vec<bool> = vec![true; n as usize];
        for num in nums {
            if num >= 1 && num <= n {
                marks[num as usize - 1] = false;
            }
        }
        for idx in 0..n {
            if marks[idx as usize] {
                return idx + 1;
            }
        }
        /* Note that enumerate() is time-consuming
        for (idx, mark) in marks.iter().enumerate() {
            if *mark {
                return idx as i32 + 1;
            }
        }*/
        n + 1
    }
}
