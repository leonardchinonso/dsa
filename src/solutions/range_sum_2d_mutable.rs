/// https://leetcode.com/problems/range-sum-query-2d-mutable
/// company: bloomberg

struct NumMatrix {
    matrix: Vec<Vec<i32>>,
    prefix_sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut prefix_sum = vec![vec![0; matrix[0].len()]; matrix.len()];

        for i in 1..prefix_sum.len() {
            for j in 1..prefix_sum[0].len() {
                prefix_sum[i][j] = prefix_sum[i - 1][j] + prefix_sum[i - 1][j]
                    - prefix_sum[i - 1][j - 1]
                    + matrix[i - 1][j - 1]
            }
        }

        Self { matrix, prefix_sum }
    }

    fn update(&mut self, row: i32, col: i32, val: i32) {
        let (row, col) = (row as usize, col as usize);
        let diff = val - self.matrix[row][col];
        self.matrix[row][col] = val;
        for i in row + 1..self.prefix_sum.len() {
            for j in col + 1..self.prefix_sum[0].len() {
                self.prefix_sum[i][j] += diff;
            }
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        self.prefix_sum[row2 + 1][col2 + 1]
            - self.prefix_sum[row2 + 1][col1]
            - self.prefix_sum[row1][col2 + 1]
            + self.prefix_sum[row1][col1]
    }
}
