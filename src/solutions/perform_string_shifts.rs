use std::collections::VecDeque;

/// https://leetcode.com/problems/perform-string-shifts/

pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
    let mut shift_polarity = 0;
    for pair in shift {
        let (direction, amount) = (pair[0], pair[1]);
        match direction {
            0 => {
                shift_polarity += amount;
            }
            1 => {
                shift_polarity -= amount;
            }
            _ => {
                panic!("invalid direction")
            }
        }
    }

    let mut queue = VecDeque::from_iter(s.chars());

    if shift_polarity.is_positive() {
        while shift_polarity > 0 {
            let ch = queue.pop_front();
            queue.push_back(ch.unwrap());
            shift_polarity -= 1;
        }
    } else {
        while shift_polarity < 0 {
            let ch = queue.pop_back();
            queue.push_front(ch.unwrap());
            shift_polarity += 1;
        }
    }

    queue.iter().collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn string_shift_works() {
        let test_cases = vec![
            (
                String::from("abc"),
                vec![vec![0, 1], vec![1, 2]],
                String::from("cab"),
            ),
            (
                String::from("abcdefg"),
                vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]],
                String::from("efgabcd"),
            ),
        ];

        for test_case in test_cases {
            let got = super::string_shift(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
