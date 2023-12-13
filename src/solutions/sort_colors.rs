/// https://leetcode.com/problems/sort-colors/

pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut left, mut anchor, mut right) = (0, 0, nums.len() - 1);

    while anchor <= right && right > 0 {
        match nums[anchor] {
            0 => {
                nums.swap(left, anchor);
                left += 1;
                anchor += 1;
            }
            2 => {
                nums.swap(anchor, right);
                right -= 1;
            }
            _ => {
                anchor += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::sort_colors::sort_colors;

    fn check_sorted(vector: &Vec<i32>) {
        for i in 1..vector.len() {
            assert!(vector[i - 1] <= vector[i])
        }
    }

    #[test]
    fn sort_colors_optimal_works() {
        let test_cases = vec![vec![2, 0, 2, 1, 1, 0], vec![0, 0, 1, 1, 2, 2]];

        for mut test_case in test_cases {
            sort_colors(&mut test_case);
            check_sorted(&test_case);
        }
    }
}
