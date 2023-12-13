pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    if nums.is_empty() {
        return true;
    }

    let mut last_one: Option<usize> = None;
    for (idx, num) in nums.iter().enumerate() {
        if *num == 0 {
            continue;
        }

        if last_one.is_none() {
            last_one = Some(idx);
            continue;
        }

        if (idx - last_one.unwrap()) as i32 - 1 < k {
            return false;
        }

        last_one = Some(idx);
    }

    true
}

#[cfg(test)]
mod test {
    #[test]
    fn k_length_apart_works() {
        let test_cases = vec![
            (vec![1, 0, 0, 0, 1, 0, 0, 1], 2, true),
            (vec![1, 0, 0, 1, 0, 1], 2, false),
        ];

        for test_case in test_cases {
            let got = super::k_length_apart(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
