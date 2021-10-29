// 36 Valid Sudoku
use crate::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut mark = [[[0; 9]; 9]; 9];
        let mut res = [[10; 9]; 9];
        let mut cnt = [[9; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                let x = board[i][j] as usize - '1' as usize;
                if board[i][j] != '.' {
                    let chk = Solution::place_number(&mut res, &mut mark, &mut cnt, i, j, x);
                    if chk == false {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn place_number(
        res: &mut [[usize; 9]; 9],
        mark: &mut [[[usize; 9]; 9]; 9],
        cnt: &mut [[usize; 9]; 9],
        i: usize,
        j: usize,
        x: usize,
    ) -> bool {
        res[i][j] = x;
        let bi = i / 3;
        let bj = j / 3;

        if mark[i][j][x] == 1 {
            return false;
        }

        for k in 0..9 {
            if mark[i][k][x] == 0 {
                cnt[i][k] -= 1;
            }
            mark[i][k][x] = 1;
            if mark[k][j][x] == 0 {
                cnt[k][j] -= 1;
            }
            mark[k][j][x] = 1;
            let ii = bi * 3 + k / 3;
            let jj = bj * 3 + k % 3;
            if mark[ii][jj][x] == 0 {
                cnt[ii][jj] -= 1;
            }
            mark[ii][jj][x] = 1;
        }
        cnt[i][j] = 0;
        for v in 0..9 {
            mark[i][j][v] = 1;
        }
        true
    }
}
