use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 1 { return 0; }

        let mut map = HashMap::new();
        let mut visited = Vec::new();
        let mut v2 = HashMap::new();
        for i in 0..n {
            visited.push(false);
            map.entry(arr[i]).or_insert(Vec::new()).push(i);
        }
        
        let mut queue = Vec::new();
        let mut idx = 0;
        queue.push((0, 0));
        visited[0] = true;

        while idx < queue.len() {
            let (i, d) = queue[idx];
            let p1 = i - 1;
            let p2 = i + 1;
            if p1 == n - 1 || p2 == n - 1 { return d + 1; }
            if !v2.contains_key(&arr[i]) {
                for j in map.get(&arr[i]).unwrap() {
                    if *j == n - 1 { return d + 1; }
                    if i != *j && !visited[*j] {
                        queue.push((*j, d + 1));
                        visited[*j] = true;
                    }
                }
            }
            v2.insert(arr[i], true);
            if p1 < n && !visited[p1] {
                queue.push((p1, d + 1));
                visited[p1] = true;
            }
            if p2 < n && !visited[p2] {
                queue.push((p2, d + 1));
                visited[p2] = true;
            }
            idx += 1;
        }
        0
    }
}

