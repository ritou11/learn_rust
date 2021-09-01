// 40 Combination Sum II
use crate::Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut cnt = [0; 51];
        for c in candidates {
            cnt[c as usize] += 1;
        }
        let mut candidates = Vec::new();
        for i in 0..51 {
            if cnt[i] > 0 {
                candidates.push(i as i32);
            }
        }
        let mut res = vec![];
        Self::work(&candidates, &mut vec![], &mut res, target, &mut cnt, 0);
        res
    }

    fn work(
        candidates: & Vec<i32>,
        curr: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        target: i32,
        cnt: &mut [i32],
        start: usize,
    ) {
        if target < 0 {
            return;
        } else if target == 0 {
            res.push(curr.clone());
            return;
        }

        for i in start..candidates.len() {
            if cnt[candidates[i] as usize] > 0 {
                curr.push(candidates[i]);
                cnt[candidates[i] as usize] -= 1;
                Self::work(candidates, curr, res, target - candidates[i], cnt, i);
                curr.pop();
                cnt[candidates[i] as usize] += 1;
            }
        }
    }
}
