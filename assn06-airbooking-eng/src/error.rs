use std::fmt;

#[derive(Debug)]
pub enum BookingError {
    Overbooked,
    InvalidSeat,
    SeatNotBooked,
}

impl fmt::Display for BookingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BookingError::Overbooked => write!(f, "All seats are booked"),
            BookingError::InvalidSeat => write!(f, "Invalid seat number"),
            BookingError::SeatNotBooked => write!(f, "Seat was not booked"),
        }
    }
}

impl std::error::Error for BookingError {}
