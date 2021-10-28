use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits.as_bytes();
        let mut res: Vec<String> = Vec::new();
        let n = digits.len();
        if n == 0 { return res; }

        let mut ava = HashMap::new();
        ava.insert('2' as u8, vec!['a', 'b', 'c']);
        ava.insert('3' as u8, vec!['d', 'e', 'f']);
        ava.insert('4' as u8, vec!['g', 'h', 'i']);
        ava.insert('5' as u8, vec!['j', 'k', 'l']);
        ava.insert('6' as u8, vec!['m', 'n', 'o']);
        ava.insert('7' as u8, vec!['p', 'q', 'r', 's']);
        ava.insert('8' as u8, vec!['t', 'u', 'v']);
        ava.insert('9' as u8, vec!['w', 'x', 'y', 'z']);

        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut s = String::from("");
        stack.push((0, 0));
        while stack.len() > 0 {
            let state = stack.pop().unwrap();
            if state.0 == n {
                res.push(s.clone());
                s.pop();
            } else {
                let pos = state.0;
                let idx = state.1;
                let c = digits[pos];
                if idx >= ava.get(&c).unwrap().len() {
                    s.pop();
                    continue;
                }
                stack.push((state.0, idx + 1));
                s.push(ava.get(&c).unwrap()[idx]);
                stack.push((state.0 + 1, 0));
            }
        }
        res
    }
}
