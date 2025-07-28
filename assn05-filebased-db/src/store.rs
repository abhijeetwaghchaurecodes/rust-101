
use crate::record::Record;
use crate::utils::append_to_file;
use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::collections::HashMap;

pub struct KeyValueStore<'a> {
    records: Vec<Box<Record>>,
    index: HashMap<String, usize>,
    log_writer: BufWriter<std::fs::File>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> KeyValueStore<'a> {
    pub fn new(log_path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(log_path)?;
        let writer = BufWriter::new(file);
        Ok(Self {
            records: Vec::new(),
            index: HashMap::new(),
            log_writer: writer,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn insert(&mut self, key: String, value: String) -> std::io::Result<()> {
        let rec = Record::new(key.clone(), value.clone());
        let pos = self.records.len();
        self.index.insert(key.clone(), pos);
        self.records.push(rec);

        append_to_file(&mut self.log_writer, &key, &value)?;
        println!("Inserted record, total records = {}", self.records.len());
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&Record> {
        self.index.get(key).map(|&idx| &*self.records[idx])
    }

    pub fn all_records(&self) -> &[Box<Record>] {
        &self.records
    }
}

impl<'a> Drop for KeyValueStore<'a> {
    fn drop(&mut self) {
        println!("[RAII] Store is shutting down. Records in memory:");
        for rec in &self.records {
            println!("  {}", rec);
        }
        println!("[RAII] Auto-writeback complete.");
    }
}
