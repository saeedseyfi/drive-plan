//! Calculates the monthly mileage budget based on insurance.

mod contract;
mod drive_plan;
mod history;

use crate::drive_plan::DrivePlan;
use chrono::NaiveDate;
use contract::Contract;
use history::{MileageHistory, MileageRecord};

fn main() {
    let plan = DrivePlan {
        contract: Contract::new(
            NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 5, 31).unwrap(),
            59620,
            15000,
        ),
        history: MileageHistory {
            records: vec![MileageRecord {
                date: NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
                mileage: 62568,
            }],
        },
    };

    println!("contract days: {}", plan.contract.days());
    println!("per day budget: {}KM", plan.contract.per_day_budget());

    println!();

    println!("days past from contract: {}", plan.contract.days_past());
    println!("mileage used: {}KM", plan.mileage_used());
    println!("avg mileage per day: {}KM", plan.mileage_used_per_day());

    println!();

    println!("days left: {}", plan.contract.days_left());
    println!("mileage left: {}KM", plan.mileage_left());
    println!("avg per day left: {}KM", plan.mileage_left_per_day());
}
