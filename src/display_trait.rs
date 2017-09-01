// The fmt method needed to fulfill the Display
// trait takes in Formatter, which we write using the write! macro
// Because the Money<T> type uses a generic type for the amount field
// We need to specify that it also must satisfy the Display trait
use std::fmt::{Formatter, Display, Result};

// pub trait Display {
//     fn fmt(&self, &mut Formatter) -> Result<(), Error>;
// }

struct Money<T> {
    amount: T,
    currency: String
}

impl<T: Display> Display for Money<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

fn main() {
    let money = Money { amount: 42, currency: "EUR".to_string() };

    println!("Displaying money: {}", money);
}
