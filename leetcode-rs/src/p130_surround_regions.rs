use crate::Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
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