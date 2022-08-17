use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut d = HashSet::new();
        let tr = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        for w in &words {
            let mut s = String::from("");
            for c in w.chars() {
                s.push_str(tr[c as usize - 97]);
            }
            d.insert(s);
        }
        d.len() as i32
    }
}