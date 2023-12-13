/// https://leetcode.com/problems/daily-temperatures/

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut vector = vec![0_i32; temperatures.len()];
    let mut stack: Vec<(usize, i32)> = Vec::with_capacity(temperatures.len());

    for curr_pair in temperatures.into_iter().enumerate() {
        if curr_pair.0 == 0 {
            stack.push(curr_pair);
            continue;
        }

        while !stack.is_empty() && stack.last().unwrap().1 < curr_pair.1 {
            let popped = stack.pop().unwrap();
            vector[popped.0] = (curr_pair.0 - popped.0) as i32;
        }

        stack.push(curr_pair);
    }

    vector
}

#[cfg(test)]
mod test {
    #[test]
    fn daily_temperatures_works() {
        let test_cases = vec![
            (
                vec![73, 74, 75, 71, 69, 72, 76, 73],
                vec![1, 1, 4, 2, 1, 1, 0, 0],
            ),
            (vec![30, 40, 50, 60], vec![1, 1, 1, 0]),
            (vec![30, 60, 90], vec![1, 1, 0]),
        ];

        for test_case in test_cases {
            let got = super::daily_temperatures(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
