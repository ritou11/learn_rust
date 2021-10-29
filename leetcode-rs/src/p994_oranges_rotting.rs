// 994. Rotting Oranges
use crate::Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut locs: Vec<(usize, usize, usize)> = Vec::new();
        let n = grid.len();
        let mut m = 0;
        for i in 0..grid.len() {
            m = grid[i].len();
            for j in 0..grid[i].len() {
                if grid[i][j] == 2 {
                    locs.push((i, j, 0));
                }
            }
        }
        let m = m;
        let mut curr = 0;
        while curr < locs.len() {
            let i = locs[curr].0;
            let j = locs[curr].1;
            let t = locs[curr].2;
            if i + 1 < n && grid[i + 1][j] == 1 {
                grid[i + 1][j] = 2;
                locs.push((i + 1, j, t + 1));
            }
            if j + 1 < m && grid[i][j + 1] == 1 {
                grid[i][j + 1] = 2;
                locs.push((i, j + 1, t + 1));
            }
            if i - 1 < n && grid[i - 1][j] == 1 {
                grid[i - 1][j] = 2;
                locs.push((i - 1, j, t + 1));
            }
            if j - 1 < m && grid[i][j - 1] == 1 {
                grid[i][j - 1] = 2;
                locs.push((i, j - 1, t + 1));
            }
            curr += 1;
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    return -1;
                }
            }
        }
        if curr == 0 {
            0
        } else {
            locs[curr - 1].2 as i32
        }
    }
}
