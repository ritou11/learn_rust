use crate::Solution;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut b_puzzles = Vec::new();
        let mut first: Vec<u32> = Vec::new();
        for p in &puzzles {
            let mut mp: u32 = 0;
            let mut flag = true;
            for b in p.bytes() {
                if flag {
                    first.push(1 << (b - 'a' as u8));
                    flag = false;
                }
                mp |= 1 << (b - 'a' as u8);
            }
            b_puzzles.push(mp);
        }
        let mut b_words = Vec::new();
        for w in &words {
            let mut mp: u32 = 0;
            for b in w.bytes() {
                mp |= 1 << (b - 'a' as u8);
            }
            b_words.push(mp);
        }

        let mut res: Vec<i32> = Vec::new();
        let mut i = 0;
        for p in &b_puzzles {
            let mut cnt = 0;
            for w in &b_words {
                if (w | p) == *p && (first[i] | w) == *w {
                    cnt += 1;
                }
            }
            res.push(cnt);
            i += 1;
        }
        res
    }
}
