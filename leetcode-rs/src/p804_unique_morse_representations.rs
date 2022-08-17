use crate::Solution;

use std::collections::HashSet;

impl Solution {
    const TR: [&'static str; 26] = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
    
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let conv = |w: String| w.into_bytes().into_iter().map(|c| Self::TR[(c - b'a') as usize]).collect::<String>();
        words.into_iter().map(conv).collect::<HashSet<String>>().len() as i32
    }
}