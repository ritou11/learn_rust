// 834. Sum of Distances in Tree
use crate::Solution;

impl Solution {
    fn dfs(i: usize, adj: &Vec<Vec<i32>>, cnt: &mut Vec<i32>, ans: &mut Vec<i32>) {
        cnt[i] = 1;
        for a in adj[i].iter() {
            if cnt[*a as usize] > 0 {
                continue; // parent
            }
            Self::dfs(*a as usize, adj, cnt, ans);
            cnt[i] += cnt[*a as usize];
            ans[i] += ans[*a as usize] + cnt[*a as usize];
        }
    }
    fn dfs2(i: usize, adj: &Vec<Vec<i32>>, cnt: &mut Vec<i32>, ans: &mut Vec<i32>) {
        for a in adj[i].iter() {
            if cnt[*a as usize] == cnt[0] {
                continue; // parent
            }
            ans[*a as usize] = ans[i] - cnt[*a as usize] + cnt[0] - cnt[*a as usize]; // cnt[0] is n
            cnt[*a as usize] = cnt[0];
            Self::dfs2(*a as usize, adj, cnt, ans);
        }
    }
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
        let mut cnt: Vec<i32> = vec![0; n as usize];
        let mut ans: Vec<i32> = vec![0; n as usize];
        for e in edges {
            adj[e[0] as usize].push(e[1]);
            adj[e[1] as usize].push(e[0]);
        }
        Self::dfs(0, &mut adj, &mut cnt, &mut ans);
        Self::dfs2(0, &mut adj, &mut cnt, &mut ans);
        ans
    }
}
