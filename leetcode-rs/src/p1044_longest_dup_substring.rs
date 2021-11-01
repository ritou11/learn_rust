use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        if s.len() <= 1 {
            return String::from("");
        }
        // let mut tails: Vec<(usize, Vec<usize>)> = Vec::new();
        // tails.push((0, vec![]));
        let bytes = s.into_bytes();

        let mut know: HashMap<u8, Vec<usize>> = HashMap::new();
        let mut poss = vec![];
        for k in 0..bytes.len() {
            (*know.entry(bytes[k]).or_insert(Vec::<usize>::new())).push(k);
            poss.push(30000);
        }

        let mut memory = HashMap::new();
        let mut max_len = 0;
        let mut max_pos = 0;
        fn search(bytes: &Vec<u8>, k: usize, i: usize, memory: &mut HashMap<(usize, usize), Vec<usize>>, know: &HashMap<u8, Vec<usize>>, poss: &mut Vec<usize>) -> Vec<usize> {
            if i > poss[k] || k < i { return vec![]; }
            match memory.get(&(k, i)) {
                Some(res) => { res.to_vec() },
                None => {
                    println!("{:?}, {:?}", k, i);
                    let c = bytes[k];
                    let mut lst: Vec<usize> = vec![];
                    if i == 1 {
                        match know.get(&c) {
                            Some(ava) => {
                                for p in ava {
                                    if *p < k {
                                        lst.push(*p);
                                    } else {
                                        break;
                                    }
                                }
                            },
                            _ => {}
                        }
                    } else {
                        let bf = search(bytes, k - 1, i - 1, memory, know, poss);
                        for p in bf {
                            if bytes[p + i - 1] == c {
                                lst.push(p);
                            }
                        }
                    }
                    memory.insert((k, i), lst.clone());
                    lst
                }
            }
        }
        for k in 1..bytes.len() {
            let mut lst: Vec<usize> = vec![];
            let mut i = max_len + 1;
            lst = search(&bytes, k, i, &mut memory, &know, &mut poss);
            while lst.len() > 0 {
                max_len = i;
                max_pos = lst[0];
                i += 1;
                lst = search(&bytes, k, i, &mut memory, &know, &mut poss);
            }
            poss[k] = i - 1;
        }
        if max_len > 0 {
            String::from_utf8(bytes[max_pos..max_pos + max_len].to_vec()).unwrap()
        } else {
            String::from("")
        }
    }
}
