/// https://leetcode.com/problems/generate-parentheses/

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let count = n;
    let mut results = Vec::new();
    let mut curr_string = String::new();
    helper(count, count, &mut results, &mut curr_string);
    results
}

fn helper(mut open_count: i32, mut closed_count: i32, results: &mut Vec<String>, curr_string: &mut String) {
    // base cases
    if open_count < 0 || closed_count < 0 || open_count > closed_count {
        return;
    }
    if open_count == 0 && closed_count == 0 {
        results.push(curr_string.clone());
    }

    // recursive case
    curr_string.push('(');
    helper(open_count-1, closed_count, results, curr_string);
    curr_string.pop();

    curr_string.push(')');
    helper(open_count, closed_count-1, results, curr_string);
    curr_string.pop();
}

#[cfg(test)]
mod test {
    #[test]
    fn generate_parenthesis_works() {
        let test_cases = vec![
            (3, vec![String::from("((()))"), String::from("(()())"), String::from("(())()"), String::from("()(())"), String::from("()()()")]),
            (2, vec![String::from("(())"), String::from("()()")]),
            (1, vec![String::from("()")]),
        ];

        for test_case in test_cases {
            let actual = super::generate_parenthesis(test_case.0);
            assert_eq!(actual, test_case.1);
        }
    }
}