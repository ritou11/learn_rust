use crate::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut stack = Vec::new();
        let mut res = Vec::new();
        let mut curr = Vec::new();
        for i in 0..nums.len() {
            curr.push(0);
        }
        stack.push((0, nums.clone()));
        while stack.len() > 0 {
            let top = stack.pop().unwrap();
            if top.1.len() == 0 {
                res.push(curr.clone());
            } else if top.1.len() > top.0 {
                let mut next_nums = Vec::new();
                for i in &top.1 {
                    if *i != top.1[top.0] {
                        next_nums.push(*i);
                    }
                }
                curr[nums.len() - top.1.len()] = top.1[top.0];
                stack.push((top.0 + 1, top.1.clone()));
                stack.push((0, next_nums));
            }
        }
        res
    }
}
