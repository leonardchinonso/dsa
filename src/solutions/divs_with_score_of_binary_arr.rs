/// https://leetcode.com/problems/all-divisions-with-the-highest-score-of-a-binary-array/

pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }

    let mut num_ones = nums.iter().filter(|&num| num == &1).count() as i32;
    let mut num_zeroes = 0_i32;
    let mut max_sum = num_ones;
    let mut result = Vec::new();
    result.push(0);

    for (idx, num) in nums.into_iter().enumerate() {
        match num {
            0 => {
                num_zeroes += 1;
            }
            1 => {
                num_ones -= 1;
            }
            _ => {}
        }

        let curr_sum = num_zeroes + num_ones;
        // println!("SUM: {sum}");
        if max_sum == curr_sum {
            result.push((idx + 1) as i32);
        } else if max_sum < curr_sum {
            result = vec![(idx + 1) as i32];
            max_sum = curr_sum;
        }
    }

    result
}

#[cfg(test)]
mod test {
    #[test]
    fn max_score_indices_works() {
        let test_cases = vec![
            (vec![0, 0, 1, 0], vec![2, 4]),
            (vec![0, 0, 0], vec![3]),
            (vec![1, 1], vec![0]),
        ];

        for test_case in test_cases {
            let got = super::max_score_indices(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
