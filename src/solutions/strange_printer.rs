pub fn strange_printer(_s: String) -> i32 {
    0
}

#[cfg(test)]
mod test {
    #[test]
    fn strange_printer_works() {
        let test_cases = vec![("aaabbb", 2), ("aba", 2)];

        for test_case in test_cases {
            let got = super::strange_printer(test_case.0.to_string());
            assert_eq!(got, test_case.1);
        }
    }
}
