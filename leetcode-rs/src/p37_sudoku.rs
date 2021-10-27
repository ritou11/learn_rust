// 37 Sudoku Solver
use crate::Solution;

// [[".",".","9","7","4","8",".",".","."],["7",".",".",".",".",".",".",".","."],[".","2",".","1",".","9",".",".","."],[".",".","7",".",".",".","2","4","."],[".","6","4",".","1",".","5","9","."],[".","9","8",".",".",".","3",".","."],[".",".",".","8",".","3",".","2","."],[".",".",".",".",".",".",".",".","6"],[".",".",".","2","7","5","9",".","."]]

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut mark = [[[0; 9]; 9]; 9];
        let mut res = [[10; 9]; 9];
        let mut cnt = [[9; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                let x = board[i][j] as usize - '1' as usize;
                if board[i][j] != '.' {
                    Solution::place_number(&mut res, &mut mark, &mut cnt, i, j, x);
                }
            }
        }
        Solution::search(&mut res, &mut mark, &mut cnt);
        // println!("{:?}", res);
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = (res[i][j] + '1' as usize) as u8 as char;
            }
        }
    }

    fn search(res: &mut [[usize; 9]; 9], mark: &mut [[[usize; 9]; 9]; 9], cnt: &mut [[usize; 9]; 9]) -> bool {
        let mut flag = true;
        while flag {
            flag = false;
            for i in 0..9 { for j in 0..9 { if cnt[i][j] == 1 {
                flag = true;
                for v in 0..9 {
                    if mark[i][j][v] == 0 {
                        Solution::place_number(res, mark, cnt, i, j, v);
                        break;
                    }
            }}}}
        }
        let mut remains: Vec<(usize, usize, usize)> = Vec::new();
        for i in 0..9 { for j in 0..9 {
            if cnt[i][j] > 0 { remains.push((i, j, cnt[i][j])); }
            if cnt[i][j] == 0 && res[i][j] == 10 { return false; }
        }}
        if remains.len() == 0 { return true; }
        remains.sort_by_key(|t| t.2);
        let ni = remains[0].0; let nj = remains[0].1;
        for v in 0..9 { if mark[ni][nj][v] == 0 {
            let mut n_res = res.to_owned();
            let mut n_mark = mark.to_owned();
            let mut n_cnt = cnt.to_owned();
            Solution::place_number(&mut n_res, &mut n_mark, &mut n_cnt, ni, nj, v);
            let solved = Solution::search(&mut n_res, &mut n_mark, &mut n_cnt);
            if solved {
                for i in 0..9 { for j in 0..9 {
                    res[i][j] = n_res[i][j];
                    cnt[i][j] = n_cnt[i][j];
                    for x in 0..9 {
                        mark[i][j][x] = n_mark[i][j][x];
                }}}
                return true;
            }
        }}
        false
    }

    fn place_number(res: &mut [[usize; 9]; 9], mark: &mut [[[usize; 9]; 9]; 9], cnt: &mut [[usize; 9]; 9], i: usize, j: usize, x: usize) {
        res[i][j] = x;
        let bi = i / 3; let bj = j / 3;
        for k in 0..9 {
            if mark[i][k][x] == 0 { cnt[i][k] -= 1; }
            mark[i][k][x] = 1;
            if mark[k][j][x] == 0 { cnt[k][j] -= 1; }
            mark[k][j][x] = 1;
            let ii = bi * 3 + k / 3;
            let jj = bj * 3 + k % 3;
            if mark[ii][jj][x] == 0 { cnt[ii][jj] -= 1; }
            mark[ii][jj][x] = 1;
        }
        cnt[i][j] = 0;
        for v in 0..9 {
            mark[i][j][v] = 1;
        }
    }
}
