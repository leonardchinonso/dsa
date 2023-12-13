fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0i32;
    for num in nums {
        result ^= num
    }
    return result;
}

fn single_number_agg(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0i32, |xor_res, val| xor_res ^ val)
}

#[cfg(test)]
mod test {
    #[test]
    fn single_number_works() {
        let test_cases = vec![(vec![1, 2, 3, 2, 1], 3), (vec![7, 7, 8, 8, 2], 2)];

        for test_case in test_cases {
            let got = super::single_number(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }

    #[test]
    fn single_number_agg_works() {
        let test_cases = vec![(vec![1, 2, 3, 2, 1], 3), (vec![7, 7, 8, 8, 2], 2)];

        for test_case in test_cases {
            let got = super::single_number_agg(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
