use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct MileageRecord {
    pub date: NaiveDate,
    pub mileage: u32,
}

#[derive(Debug)]
pub struct MileageRecords {
    pub records: Vec<MileageRecord>,
}

impl MileageRecords {
    pub fn new() -> MileageRecords {
        MileageRecords { records: vec![] }
    }

    pub fn add(&mut self, record: MileageRecord) {
        self.records.push(record);
        self.records.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn get_latest_mileage(&self) -> Result<u32, &str> {
        match self.records.last() {
            Some(record) => Ok(record.mileage),
            None => Err("No mileage records found."),
        }
    }
}
