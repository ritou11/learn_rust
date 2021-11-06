impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let ex = if n % 2 == 0 { 0 } else { 1 };
        for i in 0..n/2 {
            for j in 0..n/2 + ex {
                let tmp = matrix[n-j-1][i];
                matrix[n-j-1][i] = matrix[n-i-1][n-j-1];
                matrix[n-i-1][n-j-1] = matrix[j][n-i-1];
                matrix[j][n-i-1] = matrix[i][j];
                matrix[i][j] = tmp;
            }
        }
    }
}