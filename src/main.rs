//! Calculates the monthly mileage budget based on insurance.

use chrono::{NaiveDate, Utc};
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Contract {
    start: NaiveDate,
    end: NaiveDate,
    mileage_start: u32,
    mileage_allowance: u32,
}

impl Contract {
    fn new(
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

    fn days(&self) -> i32 {
        i32::try_from(self.end.sub(self.start).num_days()).unwrap()
    }

    fn days_past(&self) -> i32 {
        let today = Utc::now().date_naive();
        i32::try_from(today.sub(self.start).num_days()).unwrap()
    }

    fn days_left(&self) -> i32 {
        let today = Utc::now().date_naive();
        let left_days = i32::try_from(self.end.sub(today).num_days()).unwrap();
        let contract_days = self.days();

        if left_days > contract_days {
            return contract_days;
        }
        left_days
    }

    fn per_day_budget(&self) -> u32 {
        self.mileage_allowance / u32::try_from(self.days()).unwrap()
    }

    fn mileage_used(&self, current_mileage: u32) -> u32 {
        current_mileage - self.mileage_start
    }

    fn mileage_left(&self, current_mileage: u32) -> i32 {
        i32::try_from(self.mileage_start)
            .unwrap()
            .add(i32::try_from(self.mileage_allowance).unwrap())
            .sub(i32::try_from(current_mileage).unwrap())
    }

    fn mileage_used_per_day(&self, current_mileage: u32) -> u32 {
        let past = self.days_past();

        if past < 0 {
            return 0;
        }

        self.mileage_used(current_mileage) / u32::try_from(past).unwrap()
    }

    fn mileage_left_per_day(&self, current_mileage: u32) -> i32 {
        self.mileage_left(current_mileage) / self.days_left()
    }
}

#[derive(Debug)]
struct Trip {
    start: NaiveDate,
    end: NaiveDate,
    estimated_mileage: i32,
}

#[derive(Debug)]
struct Status {
    contract: Contract,
    history: Vec<(NaiveDate, u32)>,
    trips: Vec<Trip>,
}

impl Status {
    fn get_current_mileage(&self) -> u32 {
        let history = {
            let mut history = self.history.clone();
            history.sort_by(|a, b| a.0.cmp(&b.0));
            history
        };

        history.last().unwrap().1
    }

    fn mileage_used(&self) -> u32 {
        let current_mileage = self.get_current_mileage();
        self.contract.mileage_used(current_mileage)
    }

    fn mileage_left(&self) -> i32 {
        let current_mileage = self.get_current_mileage();
        self.contract.mileage_left(current_mileage)
    }

    fn mileage_used_per_day(&self) -> u32 {
        let current_mileage = self.get_current_mileage();
        self.contract.mileage_used_per_day(current_mileage)
    }

    fn mileage_left_per_day(&self) -> i32 {
        let current_mileage = self.get_current_mileage();
        self.contract.mileage_left_per_day(current_mileage)
    }
}

fn main() {
    let status = Status {
        contract: Contract::new(
            NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 5, 31).unwrap(),
            59620,
            15000,
        ),
        history: vec![
            (NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(), 62568),
            (NaiveDate::from_ymd_opt(2023, 8, 3).unwrap(), 85000),
        ],
        trips: vec![],
    };

    println!("contract days: {}", status.contract.days());
    println!("per day budget: {}KM", status.contract.per_day_budget());

    println!();

    println!("days past from contract: {}", status.contract.days_past());
    println!("mileage used: {}KM", status.mileage_used());
    println!("avg mileage per day: {}KM", status.mileage_used_per_day());

    println!();

    println!("days left: {}", status.contract.days_left());
    println!("mileage left: {}KM", status.mileage_left());
    println!("avg per day left: {}KM", status.mileage_left_per_day());
}
