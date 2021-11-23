use crate::Solution;

impl Solution {
    pub fn find(fa: &mut Vec<usize>, x: usize) -> usize {
        if x == fa[x] {
            x
        } else {
            fa[x] = Solution::find(fa, fa[x]);
            fa[x]
        }
    }
    pub fn union(fa: &mut Vec<usize>, cnt: &mut Vec<usize>, x: usize, y: usize) -> usize {
        let a = Solution::find(fa, x);
        let b = Solution::find(fa, y);
        if a == b { cnt[a] }
        else {
            cnt[a] += cnt[b];
            fa[b] = a;
            cnt[a]
        }
    }
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mx = 0;
        for num in &nums {
            mx = mx.max(*num);
        }

        let mut fa = vec![0; mx as usize + 1];
        let mut cnt = vec![0; mx as usize + 1];
        for i in 0..=mx as usize { fa[i] = i; }
        for i in 0..n { cnt[nums[i] as usize] = 1; }
        
        let mut res = 1;
        for i in 0..n {
            let num = nums[i] as usize;
            let mut j = 2;
            while j * j <= num {
                if num % j == 0 {
                    res = res.max(Solution::union(&mut fa, &mut cnt, num, j));
                    res = res.max(Solution::union(&mut fa, &mut cnt, num, num / j));
                }
                j += 1;
            }
        }
        res as i32
    }
}
