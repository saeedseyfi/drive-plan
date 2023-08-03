//! Calculates the monthly mileage budget based on insurance.

use chrono::{NaiveDate, Utc};
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Contract {
    start: NaiveDate,
    end: NaiveDate,
    mileage_start: u64,
    mileage_allowance: u64,
}

impl Contract {
    fn new(
        start: NaiveDate,
        end: NaiveDate,
        mileage_start: u64,
        mileage_allowance: u64,
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

    fn days(&self) -> i64 {
        self.end.sub(self.start).num_days()
    }

    fn days_past(&self) -> i64 {
        let today = Utc::now().date_naive();
        i64::try_from(today.sub(self.start).num_days()).unwrap()
    }

    fn days_left(&self) -> i64 {
        let today = Utc::now().date_naive();
        let left_days = i64::try_from(self.end.sub(today).num_days()).unwrap();
        let contract_days = self.days();

        if left_days > contract_days {
            return contract_days;
        }
        left_days
    }

    fn per_day_budget(&self) -> u64 {
        self.mileage_allowance / u64::try_from(self.days()).unwrap()
    }

    fn mileage_used(&self, current_mileage: u64) -> u64 {
        current_mileage - self.mileage_start
    }

    // todo test
    fn mileage_left(&self, current_mileage: u64) -> i64 {
        i64::try_from(self.mileage_start)
            .unwrap()
            .add(i64::try_from(self.mileage_allowance).unwrap())
            .sub(i64::try_from(current_mileage).unwrap())
    }

    fn used_per_day(&self, current_mileage: u64) -> u64 {
        let past = self.days_past();

        if past < 0 {
            return 0;
        }

        self.mileage_used(current_mileage) / u64::try_from(past).unwrap()
    }

    fn left_per_day(&self, current_mileage: u64) -> i64 {
        self.mileage_left(current_mileage) / self.days_left()
    }
}

#[derive(Debug)]
struct HistoryRecord {
    when: NaiveDate,
    mileage: u64,
}

#[derive(Debug)]
struct Plan {
    start: NaiveDate,
    end: NaiveDate,
    estimated_mileage: i64,
}

#[derive(Debug)]
struct Status {
    history: Vec<HistoryRecord>,
    plans: Vec<Plan>,
}

fn main() {
    let insurance = Contract::new(
        NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        NaiveDate::from_ymd_opt(2024, 5, 31).unwrap(),
        59620,
        15000,
    );
    let history = {
        let mut history = vec![
            HistoryRecord {
                when: insurance.start,
                mileage: insurance.mileage_start,
            },
            HistoryRecord {
                when: NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
                mileage: 62568,
            },
            HistoryRecord {
                when: NaiveDate::from_ymd_opt(2023, 8, 3).unwrap(),
                mileage: 85000,
            },
        ];

        history.sort_by(|a, b| a.when.cmp(&b.when));

        history
    };

    let current_mileage = history.last().unwrap().mileage;

    println!("insurance days: {}", insurance.days());
    println!("per day budget: {}KM", insurance.per_day_budget());

    println!();

    println!("days past: {}", insurance.days_past());
    println!(
        "mileage used: {}KM",
        insurance.mileage_used(current_mileage)
    );
    println!(
        "avg mileage per day: {}KM",
        insurance.used_per_day(current_mileage)
    );

    println!();

    println!("days left: {}", insurance.days_left());
    println!(
        "mileage left: {}KM",
        insurance.mileage_left(current_mileage)
    );
    println!(
        "avg per day left: {}KM",
        insurance.left_per_day(current_mileage)
    );
}
