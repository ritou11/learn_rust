/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
struct CombinationIterator {
    stack: Vec<(usize, usize)>,
    curr: String,
    length: usize,
    chars: Vec<char>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        let mut stack = Vec::new();
        let curr = String::new();
        let chars: Vec<char> = characters.chars().collect();
        stack.push((0, 0));
        CombinationIterator {
            stack: stack,
            curr: curr,
            length: combinationLength as usize,
            chars: chars,
        }
    }
    
    fn next(&mut self) -> String {
        while self.stack.len() > 0 {
            let top = self.stack.pop().unwrap();
            if self.stack.len() == self.length {
                return self.curr.clone()
            } else if top.0 + top.1 < self.chars.len() {
                if top.0 > 0 { self.curr.pop(); }
                self.curr.push(self.chars[top.0 + top.1]);
                self.stack.push((top.0 + 1, top.1));
                self.stack.push((0, top.0 + top.1 + 1));
            } else if top.0 > 0 {
                self.curr.pop();
            }
        }
        String::new()
    }
    
    fn has_next(&self) -> bool {
        if self.stack.len() <= 0 { return false; }
        for i in 0..self.stack.len() {
            if i + self.chars.len() - self.stack[i].1 - self.stack[i].0 >= self.length { return true; }
        }
        false
    }
}

