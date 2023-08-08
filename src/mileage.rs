use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct MileageRecord {
    pub(crate) date: NaiveDate,
    pub(crate) mileage: u32,
}

#[derive(Debug)]
pub struct MileageRecords {
    pub(crate) records: Vec<MileageRecord>,
}

impl MileageRecords {
    pub(crate) fn new() -> MileageRecords {
        MileageRecords { records: vec![] }
    }

    pub(crate) fn add(&mut self, record: MileageRecord) -> &mut Self {
        self.records.push(record);
        self.records.sort_by(|a, b| a.date.cmp(&b.date));
        self
    }

    pub(crate) fn get_latest_mileage(&self) -> Result<u32, &str> {
        match self.records.last() {
            Some(record) => Ok(record.mileage),
            None => Err("No mileage records found."),
        }
    }
}
