use std::cmp::{Ordering, Ordering::*};

pub fn ordering_to_string(ord: Ordering) -> String {
    match ord {
        Greater => String::from("Greater"),
        Less => String::from("Less"),
        Equal => String::from("Equal"),
    }
}
