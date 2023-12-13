pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let (mut p1, mut p2, mut carry) = ((arr1.len() - 1) as isize, (arr2.len() - 1) as isize, 0i32);
    let mut stack = Vec::new();

    while p1 >= 0 || p2 >= 0 || carry != 0 {
        let v1 = if p1 >= 0 { arr1[p1 as usize] } else { 0 };
        let v2 = if p2 >= 0 { arr2[p2 as usize] } else { 0 };

        carry += v1 + v2;
        stack.push(carry & 1);
        carry = -(carry >> 1);

        p1 -= 1;
        p2 -= 1;
    }

    let mut result = Vec::new();

    // remove the least significant bits if they're zeroes
    while let Some(val) = stack.pop() {
        if val != 0 {
            result.push(val);
            break;
        }
    }

    while let Some(val) = stack.pop() {
        result.push(val);
    }

    if result.is_empty() {
        vec![0]
    } else {
        result
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn add_negabinary_works() {
        let test_cases = vec![
            (vec![1, 1, 1, 1, 1], vec![1, 0, 1], vec![1, 0, 0, 0, 0]),
            (vec![0], vec![0], vec![0]),
            (vec![0], vec![1], vec![1]),
        ];

        for test_case in test_cases {
            let got = super::add_negabinary(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
