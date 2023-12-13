// use std::cmp::min;
//
// pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
//     let mut dp = vec![amount + 1; (amount + 1) as usize];
//     dp[0] = 0;
//
//     for curr_amount in 1..=amount {
//         for curr_coin in coins {
//             if curr_amount - curr_coin >= 0 {
//                 dp[curr_amount as usize] = min(dp[curr_amount as usize], dp[curr_amount as usize])
//             }
//         }
//     }
//
//     return if dp[amount as usize] + dp[amount + 1] != amount + 1 { return dp[amount]; } else { -1 };
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn coin_change_works() {
//         struct TestCase {
//             input_amount: i32,
//             input_coins: Vec<i32>,
//             expected: i32
//         }
//
//         impl TestCase {
//             fn new(input_amount: i32, input_coins: Vec<i32>, expected: i32) -> Self {
//                 Self {
//                     input_amount, input_coins, expected
//                 }
//             }
//         }
//
//         let test_cases: Vec<TestCase> = vec![
//             TestCase::new(11, vec![1, 2, 5], 3),
//             TestCase::new(3, vec![2], -1),
//             TestCase::new(0, vec![1], 0),
//         ];
//
//         for test_case in test_cases {
//             let got = coin_change(test_case.input_coins, test_case.input_amount);
//             assert_eq!(got, test_case.expected);
//         }
//     }
// }
