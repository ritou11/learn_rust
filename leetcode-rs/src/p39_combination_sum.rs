// 39 Combination Sum
use crate::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nbs = [false; 201];
        for i in candidates.iter() {
            nbs[*i as usize] = true;
        }
        let mut sorted = Vec::new();
        for (i, b) in nbs.iter().enumerate() {
            if *b {
                sorted.push(i as i32);
            }
        }
        return Solution::work(&sorted, target);
    }

    fn work(sorted_cdds: &[i32], target: i32) -> Vec<Vec<i32>> {
        if sorted_cdds.len() < 1 {
            return vec![];
        }
        if sorted_cdds.len() == 1 {
            if target % sorted_cdds[0] == 0 {
                return vec![vec![sorted_cdds[0]; (target/sorted_cdds[0]) as usize]];
            }
            return vec![];
        }
        // len >= 2
        let left = target - sorted_cdds[0];
        if left == 0 {
            return vec![vec![target]];
        }
        if left < 0 {
            return vec![];
        }

        let mut res = Solution::work(sorted_cdds, left);
        for v in &mut res {
            v.push(sorted_cdds[0]);
        }
        let r2 = Solution::work(&sorted_cdds[1..], target);
        res.extend(r2);

        return res;
    }
}
