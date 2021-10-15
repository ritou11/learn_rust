// 18 4Sum
use crate::Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut res = Vec::new();
        if n < 4 { return res; }
        for i in 0..=n - 4 {
            if i > 0 && nums[i] == nums[i-1] { continue; }
            for j in i+1..=n - 3 {
                if j > i+1 && nums[j] == nums[j-1] { continue; }
                let mut l = j + 1;
                let mut r = n - 1;
                let remain = target - nums[i] - nums[j];
                while l < r {
                    if nums[l] + nums[r] < remain { l += 1; }
                    else if nums[l] + nums[r] > remain { r -= 1; }
                    else {
                        res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l + 1] { l += 1; }
                        while l < r && nums[r] == nums[r - 1] { r -= 1; }
                    }
                }
            }
        }
        res
    }
}
