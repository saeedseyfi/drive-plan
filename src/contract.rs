use chrono::{Datelike, NaiveDate, Utc};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Contracts {
    pub contracts: Vec<Contract>,
}

impl Contracts {
    pub fn new() -> Contracts {
        Contracts { contracts: vec![] }
    }

    pub fn add(&mut self, contract: Contract) {
        self.contracts.push(contract);
    }

    pub fn get(&self, id: String) -> Option<&Contract> {
        self.contracts.iter().find(|c| c.id == id)
    }
}

#[derive(Debug)]
pub struct Contract {
    pub id: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub mileage_start: u32,
    pub mileage_allowance: u32,
}

impl Contract {
    pub fn new(
        id: String,
        start: NaiveDate,
        end: NaiveDate,
        mileage_start: u32,
        mileage_allowance: u32,
    ) -> Result<Contract, &'static str> {
        if end < start {
            return Err("Contract end date must be after the start date.");
        }

        Ok(Contract {
            id,
            start,
            end,
            mileage_start,
            mileage_allowance,
        })
    }

    pub fn days(&self) -> Result<u32, &'static str> {
        let days = (self.end - self.start).num_days();
        if days < 0 {
            Err("Contract duration cannot be negative.")
        } else {
            days.try_into()
                .map_err(|_| "Days do not fit into u32 bounds.")
        }
    }

    pub fn is_not_started(&self) -> bool {
        let today = Utc::now().date_naive();
        (today - self.start).num_days() < 0
    }

    pub fn is_finished(&self) -> bool {
        let today = Utc::now().date_naive();
        (self.end - today).num_days() < 0
    }

    pub fn is_during_period(&self) -> bool {
        !(self.is_not_started() || self.is_finished())
    }

    fn assert_during_period(&self) {
        if !self.is_during_period() {
            panic!("Cases where contract is not started or already finished is not correctly supported.")
        }
    }

    /// Makes sure today is within the contract period before returning the current date.
    /// This method is supposed to be used when we want to get the contract status based current date, but if the contract is already ended or not even started the calculations should bail.
    /// Use the "history" (for passed contract) or "predictions" (for oncoming) contracts.
    fn asserted_today(&self) -> NaiveDate {
        self.assert_during_period();

        Utc::now().date_naive()
    }

    pub fn days_past(&self) -> Result<u32, &'static str> {
        let today = self.asserted_today();
        (today - self.start)
            .num_days()
            .try_into()
            .map_err(|_| "Days passed do not fit into u32 bounds.")
    }

    pub fn days_left(&self) -> Result<u32, &'static str> {
        let today = self.asserted_today();
        (self.end - today)
            .num_days()
            .try_into()
            .map_err(|_| "Days left do not fit into u32 bounds.")
    }

    pub fn per_day_budget(&self) -> Result<u32, &'static str> {
        let days = self.days()?;
        Ok(self.mileage_allowance / days)
    }

    pub fn mileage_used(&self, current_mileage: u32) -> Result<u32, &'static str> {
        if current_mileage < self.mileage_start {
            Err("Current mileage cannot be less than starting mileage.")
        } else {
            Ok(current_mileage - self.mileage_start)
        }
    }

    pub fn mileage_left(&self, current_mileage: u32) -> Result<u32, &'static str> {
        let total_allowed_mileage = self.mileage_start + self.mileage_allowance;
        (total_allowed_mileage - current_mileage)
            .try_into()
            .map_err(|_| "Mileage left do not fit into u32 bounds.")
    }

    pub fn mileage_used_per_day(&self, current_mileage: u32) -> Result<u32, &'static str> {
        let days_passed = self.days_past()?;
        let mileage_used = self.mileage_used(current_mileage)?;
        (mileage_used / days_passed)
            .try_into()
            .map_err(|_| "Mileage used do not fit into u32 bounds.")
    }

    pub fn mileage_left_per_day(&self, current_mileage: u32) -> Result<u32, &'static str> {
        let days_left = self.days_left()?;
        let mileage_left = self.mileage_left(current_mileage)?;

        (mileage_left / days_left)
            .try_into()
            .map_err(|_| "Mileage used do not fit into u32 bounds.")
    }

    fn days_past_this_week(&self) -> Result<u32, &'static str> {
        let today = self.asserted_today();
        Ok(today.weekday().num_days_from_monday())
    }

    fn days_left_this_week(&self) -> Result<u32, &'static str> {
        Ok(7 - self.days_past_this_week()?)
    }

    pub fn mileage_left_this_week(&self, current_mileage: u32) -> Result<u32, &'static str> {
        let mileage_left_per_day = self.mileage_left_per_day(current_mileage)?;
        let days_left_this_week = self.days_left_this_week()?;

        Ok(days_left_this_week * mileage_left_per_day)
    }

    fn days_past_this_month(&self) -> Result<u32, &'static str> {
        let today = self.asserted_today();
        Ok(today.day0())
    }

    fn days_left_this_month(&self) -> Result<u32, &'static str> {
        let today = self.asserted_today();
        let month = today.month();
        let days_in_month =
            NaiveDate::from_ymd_opt(today.year(), if month == 12 { 1 } else { month + 1 }, 1)
                .and_then(|d| d.pred_opt().and_then(|d| Some(d.day())));

        match days_in_month {
            Some(days) => Ok(days - self.days_past_this_month()?),
            None => Err("Unable to get days in the current month."),
        }
    }

    pub fn mileage_left_this_month(&self, current_mileage: u32) -> Result<u32, &'static str> {
        let mileage_left_per_day = self.mileage_left_per_day(current_mileage)?;
        let days_left_this_month: u32 = self.days_left_this_month()?;

        Ok(days_left_this_month * mileage_left_per_day)
    }
}
