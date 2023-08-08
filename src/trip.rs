use chrono::{NaiveDate, Utc};

#[derive(Debug)]
pub struct Trip {
    pub(crate) name: String,
    pub(crate) start: NaiveDate,
    pub(crate) end: NaiveDate,
    pub(crate) estimated_mileage: u32,
}

impl Trip {
    pub fn new(
        name: String,
        start: NaiveDate,
        end: NaiveDate,
        estimated_mileage: u32,
    ) -> Result<Trip, &'static str> {
        if end < start {
            return Err("Trip end date must be after the start date.");
        }
        Ok(Trip {
            name,
            start,
            end,
            estimated_mileage,
        })
    }

    pub fn is_past(&self) -> bool {
        let today = Utc::now().date_naive();
        self.end < today
    }
}

#[derive(Debug)]
pub struct Trips {
    trips: Vec<Trip>,
}

impl Trips {
    pub fn new() -> Trips {
        Trips { trips: vec![] }
    }

    pub fn add(&mut self, trip: Trip) {
        self.trips.push(trip);
    }

    pub fn total_upcoming_mileage(&self) -> u32 {
        self.trips
            .iter()
            .filter(|trip| !trip.is_past())
            .map(|trip| trip.estimated_mileage)
            .sum()
    }
}
