use crate::Solution;

impl Solution {
    // Runtime: 4 ms, faster than 100.00% of Rust online submissions for Surrounded Regions.
    // Memory Usage: 4.7 MB, less than 95.45% of Rust online submissions for Surrounded Regions.
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut grid = board.clone();
        let mut locs: Vec<(usize, usize)> = Vec::new();
        let n = grid.len();
        let mut m = 0;
        let mut curr = 0;
        for i in 0..n {
            m = grid[i].len();
            for j in 0..grid[i].len() {
                board[i][j] = 'X';
            }
            if grid[i][0] == 'O' { locs.push((i, 0)); }
            if grid[i][m - 1] == 'O' { locs.push((i, m - 1)); }
        }
        for j in 0..m {
            if grid[0][j] == 'O' { locs.push((0, j)); } 
            if grid[n - 1][j] == 'O' { locs.push((n - 1, j)); } 
        }        
        while curr < locs.len() {
            let i = locs[curr].0;
            let j = locs[curr].1;
            board[i][j] = 'O';
            if i + 1 < n && grid[i + 1][j] == 'O' {
                grid[i + 1][j] = 'X';
                locs.push((i + 1, j));
            }
            if j + 1 < m && grid[i][j + 1] == 'O' {
                grid[i][j + 1] = 'X';
                locs.push((i, j + 1));
            }
            if i - 1 < n && grid[i - 1][j] == 'O' {
                grid[i - 1][j] = 'X';
                locs.push((i - 1, j));
            }
            if j - 1 < m && grid[i][j - 1] == 'O' {
                grid[i][j - 1] = 'X';
                locs.push((i, j - 1));
            }
            curr += 1;
        }
    }

    // Runtime: 12 ms, faster than 45.45% of Rust online submissions for Surrounded Regions.
    // Memory Usage: 4.7 MB, less than 59.09% of Rust online submissions for Surrounded Regions.
    pub fn solve2(board: &mut Vec<Vec<char>>) {
        let mut locs: Vec<(usize, usize)> = Vec::new();
        let n = board.len();
        let mut curr = 0;
        for i in 0..board.len() {
            let m = board[i].len();
            for j in 0..board[i].len() {
                if board[i][j] == 'O' {
                    let start = curr.clone();
                    let mut leak = false;
                    locs.push((i, j));
                    while curr < locs.len() {
                        let i = locs[curr].0;
                        let j = locs[curr].1;
                        if i == 0 || i == n - 1 || j == 0 || j == m - 1 { leak = true; }
                        if i + 1 < n && board[i + 1][j] == 'O' {
                            board[i + 1][j] = '?';
                            locs.push((i + 1, j));
                        }
                        if j + 1 < m && board[i][j + 1] == 'O' {
                            board[i][j + 1] = '?';
                            locs.push((i, j + 1));
                        }
                        if i - 1 < n && board[i - 1][j] == 'O' {
                            board[i - 1][j] = '?';
                            locs.push((i - 1, j));
                        }
                        if j - 1 < m && board[i][j - 1] == 'O' {
                            board[i][j - 1] = '?';
                            locs.push((i, j - 1));
                        }
                        curr += 1;
                    }
                    if !leak {
                        for k in start..locs.len() {
                            board[locs[k].0][locs[k].1] = 'X';
                        }
                    }
                }
                if board[i][j] == '?' { board[i][j] = 'O'; }
            }
        }       
    }
}