use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut cnt = HashMap::new();
        for w in &words {
            cnt.entry(w).and_modify(|e| { *e  += 1 }).or_insert(1);
        }
        let n = words.len();
        let word_len = words[0].len();
        let window = n * word_len;
        for phase in 0..word_len.min(s.len() - window + 1) {
            let mut tail = phase + window;
            while tail <= s.len() {
                let mut tmp = HashMap::new();
                let mut failed = n;
                for i in 0..n {
                    let sub = s[tail - (i + 1) * word_len..tail - i * word_len].to_string();
                    let org = cnt.get(&sub);
                    let val = tmp.entry(sub).and_modify(|e| { *e += 1 }).or_insert(1);
                    if org.is_none() || *val > *org.unwrap() {
                        failed = i;
                        break;
                    }
                }
                if failed == n {
                    res.push((tail - window) as i32);
                    tail += word_len;
                } else {
                    tail += window - failed * word_len;
                }
            }
        }
        res
    }
}