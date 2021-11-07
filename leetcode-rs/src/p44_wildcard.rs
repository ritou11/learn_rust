use crate::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.into_bytes();
        let p = p.into_bytes();
        let (mut si, mut pi) = (0, 0);
        let mut tps = 0;
        let mut tpp = -1;
        while si < s.len() {
            if pi >= p.len() {
                if tpp >= 0 {
                    tps += 1;
                    si = tps;
                    pi = tpp as usize + 1;
                    continue;
                } else {
                    return false;
                }
            }
            if s[si] == p[pi] || p[pi] == '?' as u8 {
                si += 1;
                pi += 1;
            } else if p[pi] == '*' as u8 {
                tpp = pi as i32;
                tps = si;
                pi += 1;
            } else if tpp >= 0 {
                tps += 1;
                si = tps;
                pi = tpp as usize + 1;
            } else { return false; }
        }
        while pi < p.len() && p[pi] == '*' as u8 {
            pi += 1;
        }
        pi == p.len()
    }
}
