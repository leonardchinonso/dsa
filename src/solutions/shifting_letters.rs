/// https://leetcode.com/problems/shifting-letters/

pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
    let rev_shifts = reverse_accumulator(shifts);

    let mut pos = 0usize;
    let mut res = String::new();
    for ch in s.chars() {
        res.push(shift(ch, rev_shifts[pos]));
        pos += 1;
    }

    res
}

fn shift(ch: char, offset: u64) -> char {
    let mut v = (ch as u64 - 'a' as u64) + offset;
    v = v % 26;
    (u8::try_from(v).unwrap() + 'a' as u8) as char
}

fn reverse_accumulator(arr: Vec<i32>) -> Vec<u64> {
    let mut accum = vec![0u64; arr.len()];
    let n = arr.len();

    let mut pos = n as i32 - 1;
    let mut i = n as i32 - 1;
    while i >= 0 {
        if i == n as i32 - 1 {
            accum[pos as usize] = arr[n - 1] as u64;
            pos -= 1;
            i -= 1;
            continue;
        }
        accum[pos as usize] = accum[pos as usize + 1] + arr[i as usize] as u64;
        i -= 1;
        pos -= 1;
    }
    accum
}

#[cfg(test)]
mod test {
    #[test]
    fn shift_works() {
        let test_cases = vec![('a', 1, 'b'), ('y', 2, 'a'), ('z', 1, 'a'), ('a', 17, 'r')];

        for test_case in test_cases {
            let got = super::shift(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }

    #[test]
    fn reverse_accumulator_test() {
        let test_cases = vec![(vec![3, 5, 9], vec![17, 14, 9])];

        for test_case in test_cases {
            let got = super::reverse_accumulator(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }

    #[test]
    fn shifting_letters_works() {
        let test_cases = vec![(String::from("abc"), vec![3, 5, 9], String::from("rpl"))];

        for test_case in test_cases {
            let got = super::shifting_letters(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
