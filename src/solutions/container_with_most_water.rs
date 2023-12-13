use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut area, mut left, mut right) = (0_i32, 0_usize, height.len() - 1);

    while left < right {
        area = max(area, calc_area(left, right, &height));
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    area
}

pub fn calc_area(l: usize, r: usize, height: &[i32]) -> i32 {
    let width = r - l;
    let length = min(height[l], height[r]);
    length * width as i32
}

#[cfg(test)]
mod test {
    #[test]
    fn max_area_works() {
        let test_cases = vec![(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49), (vec![1, 1], 1)];

        for test_case in test_cases {
            let got = super::max_area(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
