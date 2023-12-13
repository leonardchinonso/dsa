#[allow(unused)]
enum LookingFor {
    Left,
    Right,
}

use std::cmp::Ordering;

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let i = nums
        .binary_search_by(|n| {
            if n < &target {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap_err();
    if i == nums.len() || nums[i] != target {
        return vec![-1, -1];
    }
    let j = nums
        .binary_search_by(|n| {
            if n <= &target {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap_err();
    vec![i as i32, (j - 1) as i32]
}

// pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     if nums.is_empty() { return vec![-1, -1]; }
//
//     let left_index = find_index(&nums, target, LookingFor::Left);
//     return match left_index {
//         None => vec![-1, -1],
//         Some(left_index) => {
//             // PS: it is guaranteed that if left exists, right exists, so just unwrap
//             let right_index = find_index(&nums, target, LookingFor::Right).unwrap();
//             vec![left_index, right_index]
//         }
//     }
// }

fn _find_index(nums: &Vec<i32>, target: i32, looking_for: LookingFor) -> Option<i32> {
    let (mut start, mut end) = (0, nums.len() - 1);

    while start <= end {
        let mid = (start + end) / 2;
        if nums[mid] == target {
            match looking_for {
                LookingFor::Left => {
                    if start == mid || nums[mid - 1] < target {
                        return Some(mid as i32);
                    }
                    end = mid - 1;
                }
                LookingFor::Right => {
                    if end == mid || nums[mid + 1] > target {
                        return Some(mid as i32);
                    }
                    start = mid + 1;
                }
            }
        } else if nums[mid] > target {
            if mid == 0 {
                break;
            }
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod test {
    #[test]
    fn search_range_works() {
        let test_cases = vec![
            (vec![5, 7, 7, 8, 8, 10], 8, vec![3, 4]),
            (vec![5, 7, 7, 8, 8, 10], 6, vec![-1, -1]),
            (vec![], 0, vec![-1, -1]),
            (vec![1], 0, vec![-1, -1]),
            (vec![1], 1, vec![0, 0]),
        ];

        for test_case in test_cases {
            let got = super::search_range(test_case.0, test_case.1);
            assert_eq!(got, test_case.2);
        }
    }
}
