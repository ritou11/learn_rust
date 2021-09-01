// 39 Combination Sum
use crate::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::work(&candidates, &mut vec![], &mut res, target, 0);
        res
    }

    fn work(candidates: & Vec<i32>, curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, target: i32, start: usize) {
        if target < 0 {
            return;
        } else if target == 0 {
            res.push(curr.clone());
            return;
        }

        for i in start..candidates.len() {
            curr.push(candidates[i]);
            Self::work(candidates, curr, res, target - candidates[i], i);
            curr.pop();
        }
    }
}
