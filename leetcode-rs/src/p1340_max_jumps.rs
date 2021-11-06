use crate::Solution;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut known = Vec::new();
        for _ in 0..arr.len() {
            known.push(-1);
        }
        let mut res = 1;
        for i in 0..arr.len() {
            res = res.max(Solution::search(&arr, d as usize, i, &mut known));
        }
        res
    }
    pub fn search(arr: &Vec<i32>, d: usize, start: usize, known: &mut Vec<i32>) -> i32 {
        if known[start] >= 0 { return known[start]; }
        let mut res = 1;
        for i in 1..d+1 {
            let next = start + i;
            if next >= arr.len() || arr[next] >= arr[start] { break; }
            res = res.max(Solution::search(arr, d, next, known) + 1);
        }
        for i in 1..d+1 {
            let next = start - i;
            if next >= arr.len() || arr[next] >= arr[start] { break; }
            res = res.max(Solution::search(arr, d, next, known) + 1);
        }
        known[start] = res;
        res
    }
}
