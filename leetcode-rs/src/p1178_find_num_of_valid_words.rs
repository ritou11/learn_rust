use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut maps = Vec::new();
        let mut first = Vec::new();
        let mut res: Vec<i32> = Vec::new();
        for p in &puzzles {
            let mut mp = HashMap::new();
            let mut flag = true;
            for c in p.chars() {
                if flag {
                    first.push(c);
                    flag = false;
                }
                mp.insert(c, true);
            }
            maps.push(mp);
            res.push(0);
        }
        
        let mut word_maps = Vec::new();
        for w in &words {
            let mut mp = HashMap::new();
            let mut flag = true;
            for c in w.chars() {
                mp.insert(c, true);
            }
            word_maps.push(mp);
        }
        
        let mut i = 0;
        for p in &puzzles {
            let mut cnt = 0;
            let mut j = 0;
            for w in &words {
                if word_maps[j].get(&first[i]).is_some() {
                    let mut success = true;
                    for c in w.chars() {
                        if maps[i].get(&c).is_none() {
                            success = false;
                            break;
                        }
                    }
                    if success {
                        cnt += 1;
                    }
                }
                j += 1;
            }
            res[i] = cnt;
            i += 1;
        }
        res
    }
}
