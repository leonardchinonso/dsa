mod sorting_approach {
    use std::cmp::max;
    use std::collections::HashMap;

    pub fn maximum_sum(mut nums: Vec<i32>) -> i32 {
        let mut hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut max_sum = -1i32;

        nums.sort_by(|a, b| b.cmp(a));

        for num in nums {
            let sum = super::sum_digits(num);
            match hashmap.get_mut(&sum) {
                Some(arr) => match arr.len() {
                    1 => {
                        arr.push(num);
                        max_sum = max(max_sum, arr[0] + arr[1]);
                    }
                    _ => {}
                },
                None => {
                    hashmap.insert(sum, vec![num]);
                }
            };
        }

        max_sum
    }
}

fn sum_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

#[cfg(test)]
mod test {
    #[test]
    fn maximum_sum_works() {
        let test_cases = vec![(vec![18, 43, 36, 13, 7], 54), (vec![10, 12, 19, 14], -1)];

        for test_case in test_cases {
            let got = super::sorting_approach::maximum_sum(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
