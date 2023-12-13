/// https://leetcode.com/problems/maximum-swap/

pub fn maximum_swap(num: i32) -> i32 {
    if num == 0i32 {
        return 0i32;
    }

    let mut nums = array_from_i32(num);
    let suffix_maxes = build_suffix_maxes(&nums);

    for i in 0..nums.len() {
        if nums[i] != nums[suffix_maxes[i]] {
            nums.swap(i, suffix_maxes[i]);
            break;
        }
    }
    i32_from_array(nums)
}

fn array_from_i32(mut num: i32) -> Vec<i32> {
    let mut res = Vec::new();
    while num > 0i32 {
        res.push(num % 10i32);
        num = num / 10i32;
    }
    reverse(&mut res);
    res
}

fn i32_from_array(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |mut res, num| {
        res = res * 10 + num;
        res
    })
}

fn reverse(mut arr: &mut Vec<i32>) {
    let (mut i, mut j) = (0usize, arr.len() - 1);
    while i < j {
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn build_suffix_maxes(nums: &Vec<i32>) -> Vec<usize> {
    if nums.len() == 1 {
        return vec![0];
    }
    let mut max_idx = nums.len() - 1;
    let mut suffix_maxes = vec![0usize; nums.len()];
    suffix_maxes[nums.len() - 1] = max_idx;
    for i in (0..=nums.len() - 2).rev() {
        if nums[i] > nums[max_idx] {
            max_idx = i;
        }
        suffix_maxes[i] = max_idx;
    }
    suffix_maxes
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn maximum_swap_works() {
        let test_cases = vec![
            (2736, 7236),
            (9937, 9973),
            (9973, 9973),
            (1993, 9913),
            (1, 1),
        ];
        for test_case in test_cases {
            let actual = maximum_swap(test_case.0);
            assert_eq!(actual, test_case.1);
        }
    }

    #[test]
    fn array_from_i32_works() {
        let actual = array_from_i32(2736);
        assert_eq!(actual, vec![2, 7, 3, 6]);
        let actual = array_from_i32(1993);
        assert_eq!(actual, vec![1, 9, 9, 3]);
    }

    #[test]
    fn i32_from_array_works() {
        let actual = i32_from_array(vec![2, 7, 3, 6]);
        assert_eq!(actual, 2736);
        let actual = i32_from_array(vec![9, 9, 1, 3]);
        assert_eq!(actual, 9913);
    }

    #[test]
    fn reverse_works() {
        let mut vector = vec![2, 7, 3, 6];
        reverse(&mut vector);
        assert_eq!(vector, vec![6, 3, 7, 2]);
    }

    #[test]
    fn build_suffix_maxes_works() {
        let test_cases = vec![
            (vec![2, 7, 3, 6], vec![1, 1, 3, 3]),
            (vec![9, 9, 7, 3], vec![1, 1, 2, 3]),
            (vec![1, 9, 9, 3], vec![2, 2, 2, 3]),
        ];

        for test_case in test_cases {
            let actual = build_suffix_maxes(&test_case.0);
            assert_eq!(actual, test_case.1);
        }
    }
}
