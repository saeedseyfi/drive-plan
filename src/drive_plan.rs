use crate::contract::Contract;
use crate::history::MileageHistory;

// #[derive(Debug)]
// pub struct Trip {
//     start: NaiveDate,
//     end: NaiveDate,
//     estimated_mileage: i32,
// }

#[derive(Debug)]
pub struct DrivePlan {
    pub contract: Contract,
    pub history: MileageHistory,
    // pub trips: Vec<Trip>,
}

impl DrivePlan {
    pub fn mileage_used(&self) -> u32 {
        let current_mileage = self.history.get_current_mileage();

        match current_mileage {
            Ok(mileage) => self.contract.mileage_used(mileage),
            Err(_e) => self.contract.mileage_used(self.contract.mileage_start),
        }
    }

    pub fn mileage_left(&self) -> i32 {
        let current_mileage = self.history.get_current_mileage();

        match current_mileage {
            Ok(mileage) => self.contract.mileage_left(mileage),
            Err(_e) => self.contract.mileage_left(self.contract.mileage_start),
        }
    }

    pub fn mileage_used_per_day(&self) -> u32 {
        let current_mileage = self.history.get_current_mileage();

        match current_mileage {
            Ok(mileage) => self.contract.mileage_used_per_day(mileage),
            Err(_e) => self
                .contract
                .mileage_used_per_day(self.contract.mileage_start),
        }
    }

    pub fn mileage_left_per_day(&self) -> i32 {
        let current_mileage = self.history.get_current_mileage();

        match current_mileage {
            Ok(mileage) => self.contract.mileage_left_per_day(mileage),
            Err(_e) => self
                .contract
                .mileage_left_per_day(self.contract.mileage_start),
        }
    }
}
