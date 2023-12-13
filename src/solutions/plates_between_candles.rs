mod naive {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = s.chars().collect::<Vec<char>>();
        let mut result = Vec::with_capacity(queries.len());

        for query in queries {
            // count number of plates between the candles
            let mut count = 0_i32;
            if let Some((left_most_candle, right_most_candle)) =
                find_leftmost_and_rightmost_candles(query[0] as usize, query[1] as usize, &s)
            {
                for i in left_most_candle + 1..right_most_candle {
                    if s[i] == '*' {
                        count += 1
                    }
                }
            }
            result.push(count);
        }

        result
    }

    fn find_leftmost_and_rightmost_candles(
        left_bound: usize,
        right_bound: usize,
        arrangement: &[char],
    ) -> Option<(usize, usize)> {
        let mut left = left_bound;
        while arrangement[left] != '|' && left < right_bound {
            left += 1;
        }

        let mut right = right_bound;
        while arrangement[right] != '|' && right > left {
            right -= 1;
        }

        if arrangement[left] != '|' || arrangement[right] != '|' || left == right {
            None
        } else {
            Some((left, right))
        }
    }
}

mod optimal {
    enum SearchParam {
        LeftMost,
        RightMost,
    }

    fn binary_search(param: SearchParam, slice: &[usize], index: usize) -> usize {
        let (mut start, mut end) = (0_usize, slice.len() - 1);

        while start < end {
            match param {
                SearchParam::LeftMost => {
                    let mid = (start + end) / 2;
                    if slice[mid] == index {
                        return mid;
                    }
                    if slice[mid] > index {
                        end = mid;
                    } else {
                        start = mid + 1;
                    }
                }
                SearchParam::RightMost => {
                    let mid = (start + end) / 2;
                    if slice[mid] <= index {
                        start = mid + 1;
                    } else {
                        end = mid;
                    }
                }
            }
        }

        match param {
            SearchParam::LeftMost => start,
            SearchParam::RightMost => {
                if start < slice.len() && slice[start] > index {
                    start
                } else {
                    slice.len()
                }
            }
        }
    }

    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut candle_ids = Vec::with_capacity(s.len());
        for (idx, ch) in s.chars().enumerate() {
            if ch == '|' {
                candle_ids.push(idx)
            }
        }

        if candle_ids.is_empty() {
            return vec![0_i32; queries.len()];
        }

        let mut total_plates_in_between = Vec::with_capacity(queries.len());

        for query in queries {
            let (left_bound, right_bound) = (query[0] as usize, query[1] as usize);
            let leftmost_candle_id = binary_search(SearchParam::LeftMost, &candle_ids, left_bound);
            let mut rightmost_candle_id =
                binary_search(SearchParam::RightMost, &candle_ids, right_bound);

            // prevent overflow
            if rightmost_candle_id == 0 {
                total_plates_in_between.push(0);
                continue;
            }

            rightmost_candle_id -= 1;
            if rightmost_candle_id <= leftmost_candle_id {
                total_plates_in_between.push(0);
                continue;
            }
            let offset = rightmost_candle_id - leftmost_candle_id;
            let num_plates_btw_candles =
                candle_ids[rightmost_candle_id] - candle_ids[leftmost_candle_id];
            total_plates_in_between.push((num_plates_btw_candles - offset) as i32);
        }

        total_plates_in_between
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn plates_between_candles_works() {
        let test_cases = vec![
            ("**|**|***|", vec![vec![2, 5], vec![5, 9]], vec![2, 3]),
            (
                "***|**|*****|**||**|*",
                vec![
                    vec![1, 17],
                    vec![4, 5],
                    vec![14, 17],
                    vec![5, 11],
                    vec![15, 16],
                ],
                vec![9, 0, 0, 0, 0],
            ),
        ];

        for test_case in &test_cases {
            let got =
                super::naive::plates_between_candles(test_case.0.to_string(), test_case.1.clone());
            assert_eq!(got, test_case.2);
        }

        for test_case in test_cases {
            let got = super::optimal::plates_between_candles(test_case.0.to_string(), test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
