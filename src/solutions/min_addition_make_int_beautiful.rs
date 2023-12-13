/// https://leetcode.com/problems/minimum-addition-to-make-integer-beautiful/

pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
    let mut digits = Vec::new();
    let (mut val, mut digit_sum) = (n, 0_i64);

    // take each digit from n and append it to a vector
    // maintain a running sum of each digit
    while val > 0 {
        let last_digit = val % 10;
        digits.push(last_digit);
        digit_sum += last_digit;
        val /= 10;
    }
    // if the sum is less than the target, return 0
    if digit_sum <= target as i64 {
        return 0;
    }

    let mut exp = 10_i64;
    for i in 0..digits.len() {
        // subtract the current digit and check if its
        // elimination meets the invariant
        digit_sum -= digits[i];
        // if it meets the invariant, return the amount needed to
        // make up the next unit
        if digit_sum + 1 <= target as i64 {
            return exp - n % exp;
        }
        exp *= 10;
    }

    0
}

#[cfg(test)]
mod test {
    #[test]
    fn make_integer_beautiful_works() {
        let test_cases = vec![
            (16_i64, 6_i32, 4_i64),
            (467_i64, 6_i32, 33_i64),
            (1_i64, 1_i32, 0_i64),
        ];

        for test_case in test_cases {
            let got = super::make_integer_beautiful(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
