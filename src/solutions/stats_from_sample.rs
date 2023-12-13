/// https://leetcode.com/problems/statistics-from-a-large-sample/description/

fn min(nums: &[i32]) -> f64 {
    for (i, num) in nums.iter().enumerate() {
        if *num > 0 {
            return i as f64;
        }
    }
    -1f64
}

fn max(nums: &[i32]) -> f64 {
    for (i, num) in nums.iter().enumerate().rev() {
        if *num > 0 {
            return i as f64;
        }
    }
    -1f64
}

fn mean(nums: &[i32]) -> (f64, f64) {
    let mut pos = 0usize;
    let mut count = 0f64;
    let sum = nums.iter().fold(0f64, |mut sum, num| {
        count += *num as f64;
        sum += pos as f64 * *num as f64;
        pos += 1;
        sum
    });
    (count, sum / count)
}

fn median(nums: &[i32], total_count: f64) -> f64 {
    let half_count = total_count / 2f64;
    let (mut first, mut second) = (-1f64, -1f64);
    let mut curr_count = 0f64;
    for (i, num) in nums.iter().enumerate() {
        if *num == 0 {
            continue;
        }

        curr_count += *num as f64;
        if total_count % 2f64 == 0f64 {
            if first == -1f64 && curr_count >= half_count {
                first = i as f64;
            }
            if curr_count >= half_count + 1f64 {
                second = i as f64;
                return (first + second) / 2f64;
            }
        } else {
            if curr_count >= half_count + 1f64 {
                return i as f64;
            }
        }
    }

    -1f64
}

fn mode(nums: &[i32]) -> f64 {
    let (mut index, mut mode) = (0usize, 0i32);
    for (i, num) in nums.iter().enumerate() {
        if *num > mode {
            index = i;
            mode = *num;
        }
    }
    index as f64
}

pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
    let (total_count, mean_val) = mean(&count);
    vec![
        min(&count),
        max(&count),
        mean_val,
        median(&count, total_count),
        mode(&count),
    ]
}

mod single_pass {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut total = 0f64;
        let mut min = -1f64;
        let mut max = 0f64;
        let mut mean = 0f64;
        let mut median = 0f64;
        let mut mode = 0usize;
        let mut sum = 0f64;

        for (i, v) in count.iter().enumerate() {
            let num = *v as f64;
            let idx = i as f64;
            if num > 0f64 {
                total += num;
                if min == -1f64 {
                    min = idx;
                }
                max = idx;
                sum += idx * num;
                if *v > count[mode] {
                    mode = i;
                }
            }
        }

        mean = sum / total;

        if total == 1f64 {
            median = sum
        }
        let first = (total + 1f64) / 2f64;
        let second = total / 2f64 + 1f64;
        let (mut i, mut cnt) = (0usize, 0f64);
        while total > 1f64 && i < 256usize {
            if cnt < first && cnt + (count[i] as f64) >= first {
                median += i as f64 / 2f64;
            }
            if cnt < second && cnt + (count[i] as f64) >= second {
                median += i as f64 / 2f64;
            }
            cnt + (count[i] as f64);
            i += 1;
        }

        vec![min, max, mean, median, mode as f64]
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn sample_stats_works() {
        let test_cases = vec![
            // (
            //     vec![
            //         0, 1, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //     ],
            //     vec![1.00000, 3.00000, 2.37500, 2.50000, 3.00000],
            // ),
            // (
            //     vec![
            //         0, 4, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            //     ],
            //     vec![1.00000, 4.00000, 2.1818181818181817, 2.00000, 1.00000],
            // ),
            (
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![86.00000, 203.00000, 126.44444, 86.00000, 86.00000],
            ),
        ];

        for test_case in test_cases {
            let got = super::single_pass::sample_stats(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
