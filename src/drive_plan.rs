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
        self.contract.mileage_used(current_mileage)
    }

    pub fn mileage_left(&self) -> i32 {
        let current_mileage = self.history.get_current_mileage();
        self.contract.mileage_left(current_mileage)
    }

    pub fn mileage_used_per_day(&self) -> u32 {
        let current_mileage = self.history.get_current_mileage();
        self.contract.mileage_used_per_day(current_mileage)
    }

    pub fn mileage_left_per_day(&self) -> i32 {
        let current_mileage = self.history.get_current_mileage();
        self.contract.mileage_left_per_day(current_mileage)
    }
}
