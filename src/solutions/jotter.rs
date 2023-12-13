struct Foo {
    foo: i32,
}

fn can_mut() {
    let mut arr: Vec<Foo> = vec![Foo { foo: 6 }];
    arr[0].foo = 10;
}

fn count_distinct_elements_sorted(nums: &Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut border = 0;
    for i in 1..nums.len() {
        if nums[i - 1] != nums[i] {
            border += 1
        }
    }
    border + 1
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        super::can_mut()
    }

    #[test]
    fn count_distinct_elements_works() {
        let mut nums = vec![1, 1, 2, 2, 3, 4, 4, 4, 5];
        let got = super::count_distinct_elements_sorted(&mut nums);
        assert_eq!(got, 5);
    }
}
