use crate::Solution;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let mut max_len = 0;
        let mut max_pos = 0;

        let str = s.as_bytes();
        if str.iter().all(|&value| value == str[0]) {
            return s[1..s.len()].to_string();
        }

        let mut left = 0;
        for end in 1..s.len() {
            if max_len >= (end - left) {
                break;
            }
            let curr = &s[left..end];

            if s[(left + 1)..s.len()].contains(curr) && curr.len() > max_len {
                max_pos = left;
                max_len = curr.len();
            } else {
                left += 1;
            }
        }
        return s[max_pos..max_pos + max_len].to_string();
    }
}