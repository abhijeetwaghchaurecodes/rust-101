use crate::plane::{Plane, Seat};
use crate::error::BookingError;
use std::sync::{Arc, Mutex};

pub struct BookingSystem {
    pub plane: Plane,
}

impl BookingSystem {
    pub fn new(plane: Plane) -> Self {
        BookingSystem { plane }
    }

    pub fn book_seat(&self, name: String) -> Result<Seat, BookingError> {
        let mut seats = self.plane.seats.lock().unwrap();

        for seat_opt in seats.iter_mut() {
            if let Some(seat) = seat_opt {
                if seat.passenger_name.is_none() {
                    seat.passenger_name = Some(name.clone());
                    return Ok(seat.clone());
                }
            }
        }

        Err(BookingError::Overbooked)
    }

    pub fn cancel_booking(&self, seat_number: usize) -> Result<(), BookingError> {
        let mut seats = self.plane.seats.lock().unwrap();

        if seat_number == 0 || seat_number > seats.len() {
            return Err(BookingError::InvalidSeat);
        }

        match &mut seats[seat_number - 1] {
            Some(seat) => {
                if seat.passenger_name.is_some() {
                    seat.passenger_name = None;
                    Ok(())
                } else {
                    Err(BookingError::SeatNotBooked)
                }
            }
            None => Err(BookingError::InvalidSeat),
        }
    }

    pub fn show_seats(&self) {
        let seats = self.plane.seats.lock().unwrap();

        for seat_opt in seats.iter() {
            match seat_opt {
                Some(seat) => {
                    let status = match &seat.passenger_name {
                        Some(name) => format!("Booked by {}", name),
                        None => "Available".to_string(),
                    };
                    println!("Seat {} - {}", seat.number, status);
                }
                None => println!("Invalid Seat"),
            }
        }
    }
}
