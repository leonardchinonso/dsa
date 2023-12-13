pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    // create a vector with the amount+1 as upperbound
    let mut dp: Vec<i32> = vec![0; (amount as usize) + 1];
    dp[0] = 1; // we need one coin to make 0 amount

    for coin in coins {
        for curr_amount in coin..=amount {
            dp[curr_amount as usize] += dp[(curr_amount - coin) as usize]
        }
    }

    return dp[amount as usize];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn change_works() {
        struct TestCase {
            input_amount: i32,
            input_coins: Vec<i32>,
            expected: i32,
        }

        impl TestCase {
            fn new(input_amount: i32, input_coins: Vec<i32>, expected: i32) -> Self {
                Self {
                    input_amount,
                    input_coins,
                    expected,
                }
            }
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase::new(5, vec![1, 2, 5], 4),
            TestCase::new(3, vec![2], 0),
            TestCase::new(10, vec![10], 1),
        ];

        for test_case in test_cases {
            let got = change(test_case.input_amount, test_case.input_coins);
            assert_eq!(got, test_case.expected);
        }
    }
}
