pub fn add_two() -> i32 {
    for i in 10..=100000000 {
        let mut sum = 0i32;
        let digits = i.to_string();
        for d in digits.chars() {
            let n = d.to_digit(10);
            let n = n.unwrap();
            sum += n as i32;
        }
        if sum % 9 != 0 {
            return i;
        }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::add_two;

    #[test]
    fn it_works() {
        add_two();
    }
}
