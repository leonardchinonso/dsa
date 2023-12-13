/*
idea: simulation
    if the ball is on a 1, check next col is in bounds and 1 as well, else it is stuck
    if the ball is on a -1, check prev col is in bounds and -1 as well, else it is stuck
    if the ball is on a 1, move the ball to the next col in the next row
    if the ball is on a -1, move the ball to the prev col in the next row
*/

pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let (row_size, col_size) = (grid.len(), grid[0].len());
    let mut result: Vec<i32> = Vec::with_capacity(col_size);

    for ball_idx in 0..col_size {
        let mut is_stuck = false;
        let mut curr_col = ball_idx;
        for row_idx in 0..row_size {
            if grid[row_idx][curr_col] == 1 {
                if curr_col < col_size - 1 && grid[row_idx][curr_col + 1] == 1 {
                    curr_col += 1;
                } else {
                    result.push(-1);
                    is_stuck = true;
                    break;
                }
            } else {
                if curr_col > 0 && curr_col <= col_size - 1 && grid[row_idx][curr_col - 1] == -1 {
                    curr_col -= 1;
                } else {
                    result.push(-1);
                    is_stuck = true;
                    break;
                }
            }
        }
        if !is_stuck {
            result.push(curr_col as i32);
        }
    }

    result
}

#[cfg(test)]
mod test {
    #[test]
    fn find_ball_works() {
        let test_cases = vec![
            (
                vec![
                    vec![1, 1, 1, -1, -1],
                    vec![1, 1, 1, -1, -1],
                    vec![-1, -1, -1, 1, 1],
                    vec![1, 1, 1, 1, -1],
                    vec![-1, -1, -1, -1, -1],
                ],
                vec![1, -1, -1, -1, -1],
            ),
            (vec![vec![-1]], vec![-1]),
            (
                vec![
                    vec![1, 1, 1, 1, 1, 1],
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![-1, -1, -1, -1, -1, -1],
                ],
                vec![0, 1, 2, 3, 4, -1],
            ),
        ];

        for test_case in test_cases {
            let got = super::find_ball(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
