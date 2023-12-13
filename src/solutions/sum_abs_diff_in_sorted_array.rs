/// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/

mod prefix_suffix {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        let n = nums.len();
        let prefix_sum = build_prefix_sum(&nums);
        let suffix_sum = build_suffix_sum(&nums);
        let mut results = vec![0; n];

        for i in 0..n {
            let (mut left_sum, mut right_sum) = (0i32, 0i32);

            if i != 0 {
                left_sum = (i as i32 * nums[i]) - prefix_sum[i - 1];
            }

            if i != n - 1 {
                right_sum = suffix_sum[i + 1] - ((n - 1 - i) as i32 * nums[i])
            }

            results[i] = left_sum + right_sum
        }

        results
    }

    fn build_prefix_sum(nums: &Vec<i32>) -> Vec<i32> {
        let mut prefix_sum: Vec<i32> = vec![0; nums.len()];
        prefix_sum[0] = nums[0];
        for i in 1..nums.len() {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i]
        }
        prefix_sum
    }

    fn build_suffix_sum(nums: &Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut suffix_sum: Vec<i32> = vec![0; n];
        suffix_sum[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + nums[i]
        }
        suffix_sum
    }
}

mod optimal_prefix_suffix {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        let n = nums.len();
        let mut prefix_sum = 0i32;
        let mut results: Vec<i32> = vec![0; n];
        let total_sum: i32 = nums.iter().sum::<i32>();

        for i in 0..n {
            let suffix_sum: i32 = total_sum - prefix_sum - nums[i];
            let left_sum: i32 = (i as i32 * nums[i]) - prefix_sum;
            let right_sum: i32 = suffix_sum - ((n - 1 - i) as i32 * nums[i]);
            results[i] = left_sum + right_sum;
            prefix_sum += nums[i];
        }

        results
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get_sum_absolute_differences_works() {
        let test_cases = vec![
            (vec![2, 3, 5], vec![4, 3, 5]),
            (vec![1, 4, 6, 8, 10], vec![24, 15, 13, 15, 21]),
            (vec![], vec![]),
        ];

        for test_case in test_cases {
            let actual = super::prefix_suffix::get_sum_absolute_differences(test_case.0.clone());
            assert_eq!(actual, test_case.1.clone());
            let actual = super::optimal_prefix_suffix::get_sum_absolute_differences(test_case.0);
            assert_eq!(actual, test_case.1);
        }
    }
}
