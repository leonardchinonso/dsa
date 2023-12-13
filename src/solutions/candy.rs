/// https://leetcode.com/problems/candy/

pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut dist = vec![1; ratings.len()];

    for idx in 1..ratings.len() {
        if ratings[idx] > ratings[idx - 1] {
            dist[idx] = dist[idx - 1] + 1;
        }
    }

    for idx in (0..ratings.len() - 1).rev() {
        if ratings[idx] > ratings[idx + 1] {
            if dist[idx] <= dist[idx + 1] {
                dist[idx] = dist[idx + 1] + 1;
            }
        }
    }

    dist.into_iter().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn candy_works() {
        let test_cases = vec![
            (vec![1, 0, 2], 5),
            (vec![1, 2, 2], 4),
            (vec![1, 0, 1, 2, 2, 1], 11),
            (vec![1, 3, 4, 5, 2], 11),
        ];

        for test_case in test_cases {
            let got = super::candy(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
