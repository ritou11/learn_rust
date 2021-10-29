use crate::Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut nums: Vec<usize> = vec![1];
        for _ in 1..n {
            nums = Solution::say(nums);
        }
        let mut res = String::new();
        for i in 0..nums.len() {
            res.push(('0' as u8 + nums[i] as u8) as char);
        }
        res
    }
    pub fn say(s: Vec<usize>) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        let mut cnt = 0;
        for i in 0..s.len() - 1 {
            cnt += 1;
            if s[i] != s[i+1] {
                res.push(cnt);
                res.push(s[i]);
                cnt = 0;
            }
        }
        if s.len() > 0 {
            cnt += 1;
            res.push(cnt);
            res.push(s[s.len()-1]);
        }
        res
    }
}
