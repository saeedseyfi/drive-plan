use chrono::{NaiveDate, Utc};
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Contract {
    start: NaiveDate,
    end: NaiveDate,
    mileage_start: u32,
    mileage_allowance: u32,
}

impl Contract {
    pub fn new(
        start: NaiveDate,
        end: NaiveDate,
        mileage_start: u32,
        mileage_allowance: u32,
    ) -> Contract {
        if end.sub(start).num_days() < 1 {
            panic!("Contract must be at least 1 day long");
        }

        Contract {
            start,
            end,
            mileage_start,
            mileage_allowance,
        }
    }

    pub(crate) fn days(&self) -> i32 {
        i32::try_from(self.end.sub(self.start).num_days()).unwrap()
    }

    pub(crate) fn days_past(&self) -> i32 {
        let today = Utc::now().date_naive();
        i32::try_from(today.sub(self.start).num_days()).unwrap()
    }

    pub(crate) fn days_left(&self) -> i32 {
        let today = Utc::now().date_naive();
        let left_days = i32::try_from(self.end.sub(today).num_days()).unwrap();
        let contract_days = self.days();

        if left_days > contract_days {
            return contract_days;
        }
        left_days
    }

    pub(crate) fn per_day_budget(&self) -> u32 {
        self.mileage_allowance / u32::try_from(self.days()).unwrap()
    }

    pub(crate) fn mileage_used(&self, current_mileage: u32) -> u32 {
        current_mileage - self.mileage_start
    }

    pub(crate) fn mileage_left(&self, current_mileage: u32) -> i32 {
        i32::try_from(self.mileage_start)
            .unwrap()
            .add(i32::try_from(self.mileage_allowance).unwrap())
            .sub(i32::try_from(current_mileage).unwrap())
    }

    pub(crate) fn mileage_used_per_day(&self, current_mileage: u32) -> u32 {
        let past = self.days_past();

        if past < 0 {
            return 0;
        }

        self.mileage_used(current_mileage) / u32::try_from(past).unwrap()
    }

    pub(crate) fn mileage_left_per_day(&self, current_mileage: u32) -> i32 {
        self.mileage_left(current_mileage) / self.days_left()
    }
}
