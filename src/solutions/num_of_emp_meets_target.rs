pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours.iter().filter(|h| *h >= &target).count() as i32
}

#[cfg(test)]
mod test {
    #[test]
    fn number_of_employees_who_met_target_works() {
        let test_cases = vec![(vec![0, 1, 2, 3, 4], 2, 3), (vec![5, 1, 4, 2, 2], 6, 0)];

        for test_case in test_cases {
            let got = super::number_of_employees_who_met_target(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
