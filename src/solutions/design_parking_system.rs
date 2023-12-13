struct ParkingSystem {
    small_slots: i32,
    medium_slots: i32,
    big_slots: i32,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            small_slots: small,
            medium_slots: medium,
            big_slots: big,
        }
    }

    fn dec_big(&mut self) {
        self.big_slots -= 1;
    }
    fn dec_medium(&mut self) {
        self.medium_slots -= 1;
    }
    fn dec_small(&mut self) {
        self.small_slots -= 1;
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        return match car_type {
            1 => {
                if self.big_slots >= 1 {
                    self.dec_big();
                    return true;
                }
                false
            }
            2 => {
                if self.medium_slots >= 1 {
                    self.dec_medium();
                    return true;
                }
                false
            }
            3 => {
                if self.small_slots >= 1 {
                    self.dec_small();
                    return true;
                }
                false
            }
            _ => false,
        };
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let test_cases = vec![(1, 1, 0)];
        for test_case in test_cases {
            let got = ParkingSystem::new(test_case.0, test_case.1, test_case.2);
            assert_eq!(got.big_slots, test_case.0);
            assert_eq!(got.medium_slots, test_case.1);
            assert_eq!(got.small_slots, test_case.2);
        }
    }

    #[test]
    fn add_car_works() {
        let test_cases = vec![(1, true), (2, true), (3, false), (1, false)];
        let mut ps = ParkingSystem::new(1, 1, 0);
        for test_case in test_cases {
            let got = ps.add_car(test_case.0);
            assert_eq!(got, test_case.1);
        }
    }
}
