/// https://leetcode.com/problems/simplify-path/description/

pub fn simplify_path(path: String) -> String {
    // create a stack to push the strings to
    let mut stack: Vec<String> = Vec::new();

    // for each directory
    for dir in path.split("/") {
        match dir {
            "" | "." => {}
            // normally, we'd check if stack is empty before popping, but rust is rusting :(
            ".." => {
                stack.pop();
            }
            _ => stack.push(format!("/{}", dir.to_string())),
        }
    }

    if !stack.is_empty() {
        stack.into_iter().collect::<String>()
    } else {
        String::from("/")
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::simplify_path::simplify_path;

    #[test]
    fn simplify_path_test_works() {
        let test_cases = vec![
            ("/a/./b/../../c", "/c"),
            ("/home/../foo//bar/dave//", "/foo/bar/dave"),
            ("/.", "/"),
            ("/..", "/"),
        ];

        for test_case in test_cases {
            let got = simplify_path(test_case.0.to_string());
            assert_eq!(got, test_case.1);
        }
    }
}
