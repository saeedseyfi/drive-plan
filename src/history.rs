use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct MileageRecord {
    pub(crate) date: NaiveDate,
    pub(crate) mileage: u32,
}

#[derive(Debug)]
pub struct MileageHistory {
    pub(crate) records: Vec<MileageRecord>,
}

impl MileageHistory {
    pub fn get_current_mileage(&self) -> u32 {
        let records = {
            let mut records = self.records.clone();
            records.sort_by(|a, b| a.date.cmp(&b.date));
            records
        };

        records.last().unwrap().mileage
    }
}
