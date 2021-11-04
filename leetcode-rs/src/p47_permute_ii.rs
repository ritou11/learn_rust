use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut stack = Vec::new();
        let mut res = Vec::new();
        let mut curr = Vec::new();
        let mut uni = HashMap::new();
        for i in 0..nums.len() {
            curr.push(0);
            *(uni.entry(nums[i]).or_insert(0)) += 1;
        }
        let mut perp = Vec::new();
        for (num, cnt) in uni.iter() {
            perp.push((*num, *cnt));
        }
        stack.push((0, 0, perp.clone()));
        while stack.len() > 0 {
            let top = stack.pop().unwrap();
            if top.2.len() == 0 {
                res.push(curr.clone());
            } else if top.2.len() > top.0 {
                let mut next_nums = Vec::new();
                let next_num = top.2[top.0].0;
                for (num, cnt) in &top.2 {
                    if *num != next_num {
                        next_nums.push((*num, *cnt));
                    } else if *cnt > 1 {
                        next_nums.push((*num, *cnt - 1));
                    }
                }
                curr[top.1] = next_num;
                stack.push((top.0 + 1, top.1, top.2));
                stack.push((0, top.1 + 1, next_nums));
            }
        }
        res
    }
}
