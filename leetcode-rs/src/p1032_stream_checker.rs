use std::collections::HashSet;

struct StreamChecker {
    words: Vec<HashSet<String>>,
    last_char: Vec<Vec<usize>>,
    buffer: String
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut last_char = vec![Vec::new(); 26];
        let mut char_words = vec![HashSet::new(); 26];
        for word in &words {
            let c = word.chars().nth(word.len()-1).unwrap();
            let i = (c as u8 - 'a' as u8) as usize;
            last_char[i].push(word.len());
            char_words[i].insert(word.clone());
        }
        Self {
            words: char_words,
            last_char: last_char,
            buffer: String::new()
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.buffer.push(letter);
        let n = self.buffer.len();
        let c = (letter as u8 - 'a' as u8) as usize;
        for m in &self.last_char[c] {
            if *m > n { continue; }
            let tail = self.buffer[(n-m)..n].to_string();
            if self.words[c].contains(&tail) {
                return true;
            }
        }
        return false;
    }
}