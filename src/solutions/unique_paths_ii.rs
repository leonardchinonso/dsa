mod top_down_approach {
    use std::collections::HashMap;

    pub fn unique_paths_with_obstacles(obstacle_grid: &Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() {
            return 0;
        }

        let mut memo = HashMap::new();
        memo.insert((obstacle_grid.len() - 1, obstacle_grid[0].len() - 1), 1);

        dfs(0, 0, &obstacle_grid, &mut memo)
    }

    fn dfs(
        row: usize,
        col: usize,
        grid: &Vec<Vec<i32>>,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if row == grid.len() || col == grid[0].len() || grid[row][col] == 1 {
            return 0;
        }

        if let Some(val) = memo.get(&(row, col)) {
            return *val;
        }

        let res = dfs(row + 1, col, grid, memo) + dfs(row, col + 1, grid, memo);

        memo.insert((row, col), res);

        res
    }
}

mod bottom_up_approach {
    pub fn unique_paths_with_obstacles(obstacle_grid: &Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() {
            return 0;
        }

        let n = obstacle_grid[0].len();
        let mut dp: Vec<i32> = vec![0; n];
        dp[n - 1] = 1;

        for row in obstacle_grid.iter().rev() {
            for (col_idx, col_val) in row.iter().enumerate().rev() {
                if col_val == &1 {
                    dp[col_idx] = 0;
                } else if col_idx < n - 1 {
                    dp[col_idx] = dp[col_idx] + dp[col_idx + 1];
                }
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unique_paths_with_obstacles_works() {
        let test_cases = vec![
            (vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]], 2),
            (vec![vec![0, 1], vec![0, 0]], 1),
        ];

        for test_case in &test_cases {
            let got = top_down_approach::unique_paths_with_obstacles(&test_case.0);
            assert_eq!(got, test_case.1);
        }

        for test_case in &test_cases {
            let got = bottom_up_approach::unique_paths_with_obstacles(&test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
