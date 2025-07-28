
use std::fmt;

#[derive(Debug)]
pub struct Record {
    pub key: String,
    pub value: String,
}

impl Record {
    pub fn new(key: String, value: String) -> Box<Self> {
        let rec = Box::new(Record { key, value });
        println!("[Memory] Allocated Record on heap at: {:p}", &*rec);
        rec
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.key, self.value)
    }
}
