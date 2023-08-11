use crate::contract::Contracts;
use crate::mileage::MileageRecords;
use crate::trip::Trips;

#[derive(Debug)]
pub struct Vehicles {
    // vehicles: Vec<Vehicle>,
}

#[derive(Debug)]
pub struct Vehicle {
    // pub(crate) id: String,
    pub(crate) contracts: Contracts,
    pub(crate) mileage_records: MileageRecords,
    pub(crate) trips: Trips,
}

impl Vehicle {
    pub(crate) fn new(_id: String) -> Vehicle {
        Vehicle {
            // id,
            contracts: Contracts::new(),
            mileage_records: MileageRecords::new(),
            trips: Trips::new(),
        }
    }
}
