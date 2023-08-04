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
    pub fn new() -> MileageHistory {
        MileageHistory { records: vec![] }
    }

    pub fn add_record(&mut self, record: MileageRecord) {
        self.records.push(record);
        self.records.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn get_current_mileage(&self) -> Result<u32, &str> {
        let records = {
            let mut records = self.records.clone();
            records.sort_by(|a, b| a.date.cmp(&b.date));
            records
        };

        match records.last() {
            Some(record) => Ok(record.mileage),
            None => Err("No mileage records found."),
        }
    }
}
