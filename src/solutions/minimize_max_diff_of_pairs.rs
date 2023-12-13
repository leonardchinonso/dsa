/// https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/description/

pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
    if nums.is_empty() || p == 0 {
        return 0;
    }
    nums.sort();

    let (mut start, mut end) = (0, *nums.last().unwrap() - *nums.first().unwrap());
    while start < end {
        let pot_max = (start + end) / 2;
        if num_pairs_lt_or_eq_pot_max(&nums, pot_max) >= p {
            end = pot_max;
        } else {
            start = pot_max + 1;
        }
    }

    start
}

pub fn num_pairs_lt_or_eq_pot_max(nums: &Vec<i32>, val: i32) -> i32 {
    let mut count = 0i32;
    let mut i = 1usize;
    while i < nums.len() {
        if nums[i] - nums[i - 1] <= val {
            count += 1;
            i += 1;
        }
        i += 1;
    }
    count
}

#[cfg(test)]
mod test {
    #[test]
    fn minimize_max_works() {
        let test_cases = vec![
            (vec![10, 1, 2, 7, 1, 3], 2, 1),
            (vec![4, 2, 1, 2], 1, 0),
            (vec![3, 4, 2, 3, 2, 1, 2], 3, 1),
        ];

        for test_case in test_cases {
            let got = super::minimize_max(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
