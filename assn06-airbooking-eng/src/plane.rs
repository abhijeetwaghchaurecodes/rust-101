use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Seat {
    pub number: usize,
    pub passenger_name: Option<String>,
}

#[derive(Debug)]
pub struct Plane {
    pub id: String,
    pub seats: Arc<Mutex<Vec<Option<Seat>>>>,
}

impl Plane {
    pub fn new(id: &str, capacity: usize) -> Self {
        let mut empty_seats = Vec::with_capacity(capacity);
        for i in 1..=capacity {
            empty_seats.push(Some(Seat { number: i, passenger_name: None }));
        }
        Plane {
            id: id.to_string(),
            seats: Arc::new(Mutex::new(empty_seats)),
        }
    }
}
