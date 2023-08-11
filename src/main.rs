//! Mileage Budget Calculator

mod contract;
mod mileage;
mod trip;
mod user;
mod vehicle;

use crate::trip::Trip;
use crate::vehicle::Vehicle;
use chrono::NaiveDate;
use contract::Contract;
use mileage::MileageRecord;

fn main() {
    let volvo = {
        let mut vehicle = Vehicle::new(String::from("Volvo V60 CPA53U"));

        vehicle.contracts.add(
            Contract::new(
                String::from("Volvia"),
                NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 5, 31).unwrap(),
                59620,
                15000,
            )
            .unwrap(),
        );

        vehicle
            .mileage_records
            .add(MileageRecord {
                date: NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
                mileage: 62568,
            })
            .add(MileageRecord {
                date: NaiveDate::from_ymd_opt(2023, 8, 6).unwrap(),
                mileage: 62821,
            })
            .add(MileageRecord {
                date: NaiveDate::from_ymd_opt(2023, 8, 11).unwrap(),
                mileage: 62890,
            });

        vehicle
            .trips
            .add(
                Trip::new(
                    String::from("Offsite"),
                    NaiveDate::from_ymd_opt(2023, 8, 14).unwrap(),
                    NaiveDate::from_ymd_opt(2023, 8, 17).unwrap(),
                    200,
                )
                .unwrap(),
            )
            .add(
                Trip::new(
                    String::from("Offsite"),
                    NaiveDate::from_ymd_opt(2023, 9, 22).unwrap(),
                    NaiveDate::from_ymd_opt(2023, 9, 23).unwrap(),
                    240,
                )
                .unwrap(),
            );

        vehicle
    };
    let insurance = volvo.contracts.get(String::from("Volvia")).unwrap();
    let mileage = volvo.mileage_records.get_latest_mileage().unwrap();
    let trips_mileage = volvo.trips.total_upcoming_mileage();

    println!("{:#?}\n", volvo);

    println!("contract days: {}", insurance.days().unwrap());
    println!("per day budget: {}KM", insurance.per_day_budget().unwrap());

    println!();

    println!(
        "days past from contract: {}",
        insurance.days_past().unwrap()
    );
    println!(
        "mileage used: {}KM",
        insurance.mileage_used(mileage).unwrap()
    );
    println!(
        "avg mileage per day: {}KM",
        insurance.mileage_used_per_day(mileage).unwrap()
    );

    println!();

    println!("days left: {}", insurance.days_left().unwrap());
    println!(
        "mileage left: {}KM",
        insurance.mileage_left(mileage, trips_mileage).unwrap()
    );
    println!(
        "avg per day left: {}KM",
        insurance
            .mileage_left_per_day(mileage, trips_mileage)
            .unwrap()
    );

    println!();

    println!(
        "mileage left this week: {}KM",
        insurance
            .mileage_left_this_week(mileage, trips_mileage)
            .unwrap()
    );

    println!(
        "mileage left this month: {}KM",
        insurance
            .mileage_left_this_month(mileage, trips_mileage)
            .unwrap()
    );
}
