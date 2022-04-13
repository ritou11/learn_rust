use crate::Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        for _ in 0..n {
            let mut line = Vec::new();
            for _ in 0..n {
                line.push(0);
            }
            res.push(line);
        }
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut idx = 0;
        let mut pnt: (i32, i32) = (0, 0);
        let chk = |nxt: (i32, i32)| nxt.0 >= n || nxt.0 < 0 || nxt.1 >= n || nxt.1 < 0;
        for i in 0..n * n {
            res[pnt.0 as usize][pnt.1 as usize] = i + 1;
            let mut nxt = (pnt.0 + DIRS[idx].0, pnt.1 + DIRS[idx].1);
            if chk(nxt) || res[nxt.0 as usize][nxt.1 as usize] != 0 {
                idx = (idx + 1) % 4;
                nxt = (pnt.0 + DIRS[idx].0, pnt.1 + DIRS[idx].1);
            }
            pnt = nxt;
        }
        res
    }
}
