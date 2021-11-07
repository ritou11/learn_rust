impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut d = Vec::new();
        for _ in 0..n {
            d.push(false);
        }
        if s[n - 1] == '1' { return false; }
        d[n - 1] = true;
        let mut l = (n - 1) as i32;
        for i in (0..n).rev() {
            if s[i] == '0' && d[i] {
                let r = l.min(i as i32 - min_jump + 1).max(0);
                l = (i as i32 - max_jump).max(0);
                for k in (l as usize)..(r as usize) {
                    d[k] = true;
                }
                if l == 0 {
                    break;
                }
            }
        }
        d[0]
    }
}
