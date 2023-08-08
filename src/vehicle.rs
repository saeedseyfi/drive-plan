use crate::contract::Contracts;
use crate::mileage::MileageRecords;

// #[derive(Debug)]
// pub struct Trip {
//     start: NaiveDate,
//     end: NaiveDate,
//     estimated_mileage: u32,
// }
//
// #[derive(Debug)]
// struct Trips {
//     trips: Vec<Trip>
// }

#[derive(Debug)]
pub struct Vehicle {
    pub id: String,
    pub contracts: Contracts,
    pub mileage_records: MileageRecords,
    // pub trips: Trips,
}

impl Vehicle {
    pub fn new(id: String) -> Vehicle {
        Vehicle {
            id,
            contracts: Contracts::new(),
            mileage_records: MileageRecords::new(),
        }
    }
}
