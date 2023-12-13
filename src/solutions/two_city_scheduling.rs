/// https://leetcode.com/problems/two-city-scheduling
/// company

pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut costs = costs;
    costs.sort_by(|cost1, cost2| (cost1[0] - cost1[1]).cmp(&(cost2[0] - cost2[1])));

    let mut sum = 0;
    sum += (0..(costs.len() / 2)).into_iter().fold(0, |mut v, i| {
        v += costs[i][0];
        v
    });
    sum += ((costs.len() / 2)..costs.len())
        .into_iter()
        .fold(0, |mut v, i| {
            v += costs[i][1];
            v
        });
    sum
}

#[cfg(test)]
mod test {
    #[test]
    fn two_city_sched_cost_works() {
        let test_cases = vec![
            (
                vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]],
                110,
            ),
            (
                vec![
                    vec![259, 770],
                    vec![448, 54],
                    vec![926, 667],
                    vec![184, 139],
                    vec![840, 118],
                    vec![577, 469],
                ],
                1859,
            ),
            (
                vec![
                    vec![515, 563],
                    vec![451, 713],
                    vec![537, 709],
                    vec![343, 819],
                    vec![855, 779],
                    vec![457, 60],
                    vec![650, 359],
                    vec![631, 42],
                ],
                3086,
            ),
        ];

        for test_case in test_cases {
            let got = super::two_city_sched_cost(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
