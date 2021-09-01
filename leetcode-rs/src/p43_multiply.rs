// 43 Multiply Strings
use crate::Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let zero = String::from("0");
        if num1 == zero || num2 == zero {
            return zero;
        }
        // let num1 = Self::parse_to_vec(&num1);
        // let num2 = Self::parse_to_vec(&num2);

        let n = num1.len() + num2.len();
        let mut res:Vec<u32> = vec![0;n];
        for i in 0..num1.len() {
            for j in 0..num2.len() {
                // Note that char.to_digit(10) is slow
                res[i + j + 1] += ((num1.as_bytes()[i] - b'0') as u32) * ((num2.as_bytes()[j] - b'0') as u32);
            }
        }
        for i in (1..n).rev() {
            res[i - 1] += res[i] / 10;
            res[i] %= 10;
        }
        let mut arr:Vec<char> = Vec::new();
        if res[0] != 0 {
            arr.push(char::from_digit(res[0], 10).unwrap());
        }
        for i in 1..n {
            arr.push(char::from_digit(res[i], 10).unwrap());
        }
        arr.into_iter().collect()
    }

    /*
    fn parse_to_vec(num: &String) -> Vec<u32> {
        let mut res: Vec<u32> = Vec::new();
        for c in num.chars() {
            res.push(c.to_digit(10).unwrap_or(0));
        }
        res
    }
    */
}
