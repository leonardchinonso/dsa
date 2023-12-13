pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut result = 0;

    // if k is zero, the arrays must be equal
    if k == 0 {
        result = if nums1 == nums2 { 0 } else { -1 };
        return result;
    }

    let (mut diff_1, mut diff_2) = (0i32, 0i32);
    // move through the arrays and check for differences
    for i in 0..nums1.len() {
        let diff = nums1[i] - nums2[i];

        // if the number in nums2 is smaller, save the diff in diff1
        if diff.is_positive() {
            diff_1 = diff
        } else {
            // save it in diff2
            diff_2 = diff.abs()
        }
    }

    if diff_1 == diff_2 && diff_1 % k == 0 && diff_2.abs() % k == 0 {
        return (diff_1 / k) as i64;
    }

    result
}

#[cfg(test)]
mod test {
    #[test]
    fn min_operations_works() {
        let test_cases = vec![
            (vec![4, 3, 1, 4], vec![1, 3, 7, 1], 3, 2),
            (vec![3, 8, 5, 2], vec![2, 4, 1, 6], 1, -1),
        ];

        for test_case in test_cases {
            let got = super::min_operations(test_case.0, test_case.1, test_case.2);
            assert_eq!(got, test_case.3)
        }
    }
}
