/// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix

mod binary_search_approach {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let mut count = 0;
        let row_len = grid[0].len();
        for row in grid {
            let idx = first_negative_index(&row);
            count += row_len - idx.unwrap_or(0);
        }

        count as i32
    }

    fn first_negative_index(row: &[i32]) -> Option<usize> {
        None
    }
}

mod optimal_linear_search_approach {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let mut count = 0;
        let row_len = grid[0].len();
        let bound = row_len as i32 - 1;
        for row in grid {
            let (idx, cnt) = linear_count(&row, bound);
            count += cnt;
        }

        count as i32
    }

    fn linear_count(row: &[i32], mut idx: i32) -> (usize, i32) {
        // while idx >= 0 {
        //     if row[idx as usize].is_positive() {
        //         return (idx as usize + 1, (row.len() - idx) as i32);
        //     }
        //     idx -= 1;
        // }
        // (0usize, row.len() as i32)
        (0, 0)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn count_negatives_works() {
        let test_cases = vec![
            (
                vec![
                    vec![4, 3, 2, -1],
                    vec![3, 2, 1, -1],
                    vec![1, 1, -1, -2],
                    vec![-1, -1, -2, -3],
                ],
                8,
            ),
            (vec![vec![3, 2], vec![1, 0]], 0),
        ];

        for test_case in test_cases {
            let got = super::binary_search_approach::count_negatives(test_case.0.clone());
            assert_eq!(got, test_case.1);
            // let got = super::optimal_linear_search_approach::count_negatives(test_case.0);
            // assert_eq!(got, test_case.1);
        }
    }
}
