use crate::Solution;

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mx = 0;
        for num in &nums {
            mx = mx.max(*num);
        }

        let mut primes = Vec::new();
        let mut is_prime = vec![true;mx as usize + 1];
        is_prime[1] = false;
        
        for i in 2..(mx as f64).sqrt() as usize + 1 {
            if is_prime[i] {
                let mut j = i * i;
                while j <= mx as usize {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        for i in 2..mx as usize + 1 {
            if is_prime[i] { primes.push(i as i32); }
        }

        let m = primes.len();
        let mut map1 = vec![Vec::<usize>::new(); n];
        let mut map2 = vec![Vec::<usize>::new(); m];

        for i in 0..n {
            for j in 0..m {
                if primes[j] > nums[i] { break; }
                if nums[i] % primes[j] == 0 {
                    map1[i].push(j);
                    map2[j].push(i);
                }
            }
        }

        let mut visited = vec![false; n];
        let mut queue = Vec::new();
        let mut head = 0;
        let mut res = 0;
        for i in 0..n {
            if visited[i] { continue; }
            visited[i] = true;
            queue.push(i);
            let mut cnt = 0;
            while head < queue.len() {
                cnt += 1;
                let idx = queue[head];
                for v in &map1[idx] {
                    for c in &map2[*v] {
                        if !visited[*c] {
                            visited[*c] = true;
                            queue.push(*c);
                        }
                    }
                }
                head += 1;
            }
            res = res.max(cnt);
        }
        res
    }
}
