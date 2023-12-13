/// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/

mod prefix_sum {
    use std::collections::HashMap;

    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let travel = prefix_sum(travel);
        let mut hashmap: HashMap<char, [Option<usize>; 2]> = HashMap::from([
            ('G', [None, Some(0usize)]),
            ('M', [None, Some(0usize)]),
            ('P', [None, Some(0usize)]),
        ]);

        for (idx, pack) in garbage.iter().enumerate() {
            for c in pack.chars() {
                hashmap.entry(c).and_modify(|v| {
                    v[0] = Some(idx);
                    v[1] = Some(v[1].unwrap() + 1);
                });
            }
        }

        let mut total = 0i32;
        for (_, v) in hashmap {
            if v[0].is_none() {
                continue;
            }
            let (last_idx, count) = (v[0].unwrap(), v[1].unwrap() as i32);
            total += (travel[last_idx] + count);
        }

        total
    }

    fn prefix_sum(array: Vec<i32>) -> Vec<i32> {
        let mut sum = vec![0i32; array.len() + 1];
        for i in 1..sum.len() {
            sum[i] = sum[i - 1] + array[i - 1];
        }
        sum
    }
}

mod single_pass {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        garbage
            .into_iter()
            .rev()
            .zip(travel.into_iter().rev().chain(Some(0i32)))
            .fold(
                [0i32, 0i32, 0i32],
                |[mut m, mut p, mut g], (pack, distance)| {
                    for c in pack.bytes() {
                        match c {
                            b'M' => m += 1,
                            b'P' => p += 1,
                            b'G' => g += 1,
                            _ => {}
                        }
                    }
                    if m > 0i32 {
                        m += distance;
                    }
                    if p > 0i32 {
                        p += distance;
                    }
                    if g > 0i32 {
                        g += distance;
                    }
                    [m, p, g]
                },
            )
            .iter()
            .sum()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn garbage_collection_works() {
        let test_cases = vec![
            (
                vec![
                    String::from("G"),
                    String::from("P"),
                    String::from("GP"),
                    String::from("GG"),
                ],
                vec![2, 4, 3],
                21,
            ),
            (
                vec![String::from("MMM"), String::from("PGM"), String::from("GP")],
                vec![3, 10],
                37,
            ),
        ];

        for test_case in test_cases.iter() {
            let actual =
                super::prefix_sum::garbage_collection(test_case.0.clone(), test_case.1.clone());
            assert_eq!(actual, test_case.2);
        }

        for test_case in test_cases {
            let actual = super::single_pass::garbage_collection(test_case.0, test_case.1);
            assert_eq!(actual, test_case.2);
        }
    }
}
