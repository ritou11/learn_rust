use crate::Solution;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let (mut sx, mut sy) = (0, 0);
        let mut n_empty = 0;
        for i in 0..n {
            for j in 0..m {
                match grid[i][j] {
                    1 => { sx = i; sy = j; n_empty += 1; },
                    0 => { n_empty += 1; },
                    _ => {},
                }
            }
        }
        let mut map = grid.clone();
        Solution::search(&mut map, sx, sy, n_empty)
    }

    pub fn search(grid: &mut Vec<Vec<i32>>, x: usize, y:usize, n_empty: usize) -> i32 {
        if x >= grid.len() || y >= grid[0].len() { return 0; }
        match grid[x][y] {
            0 | 1 => {
                let mut cnt = 0; 
                for (nx, ny) in &[(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
                    grid[x][y] = -1;
                    cnt += Solution::search(grid, *nx, *ny, n_empty - 1);
                    grid[x][y] = 0;
                }
                cnt
            },
            2 => { if n_empty == 0 { 1 } else { 0 } }
            _ => { 0 }
        }
    }
}
