use std::collections::HashMap;

struct UndergroundSystem {
    check_ins: HashMap<i32, (String, i32)>,
    records: HashMap<(String, String), (f64, usize)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            check_ins: HashMap::new(),
            records: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_ins.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (station_out, time_out) = (station_name, t);
        let (_, (station_in, time_in)) = self.check_ins.remove_entry(&id).unwrap();
        let time_diff = (time_out - time_in) as f64;

        self.records
            .entry((station_in, station_out))
            .and_modify(|(time_taken, freq)| {
                *time_taken += time_diff;
                *freq += 1;
            })
            .or_insert((time_diff, 1));
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let (time_taken, freq) = self.records.get(&(start_station, end_station)).unwrap();
        time_taken / (*freq as f64)
    }
}
