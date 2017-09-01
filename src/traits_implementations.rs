//  Traits and Implementations
// |--------------------------|
// A TRAIT is a collection of methods defined for an unknown type: Self.
// They can access other methods declared in the same trait and be implemented for any data type.


// OOP in Rust
// - Composition over inheritance
// - You can write amplementation blocks anywhere, w/o having access to the actual type

//  Syntax for Trait blocks:
// |------------------------|
// trait TraitName {
//   fn method(&self);
// }

// An implementation would need to specify soemthing for all of these things:
// --------------------------------------------------------------------------
// impl TraitName for myType {
//   fn method(&self) {
//     // Implementation
//   }
// }

// Intro Trait - Declares an into method, giving a general way to do conversions b/w types
// pub trait Into<T> {
//     fn into(self) -> T;
// }

// pub trait Add<RHS = Self> { // Trait has a generic type RHS that needs to be equal to the Self type
//     // Any implementation needs to declare an Output type
//     type Output;
//     // Any implementation needs to implement an add method that takes a right-handed side
//     // parameter that was declared on the first line to be the same as the Self type
//     // In other words, the left-hand side and the right-hand side around the + operator need
//     // to be of the same type.
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// Common Traits:
// --------------
use std::ops::Add; // - trait - overload the + operator
// std::convert::Into - specify coversion methods from and to arbitraty types
// Display - specify how types are formatted as a string

#[derive(Debug)]
struct Money<T> {
    amount: T,
        currency: String
}

#[derive(Debug)]
struct CurrencylessMoney<T> {
    amount: T
}


//  Takaways and Tasks
// |------------------|
// - Traits are like interfaces in OO languages: they allow giving types additional
//   functionalities  in a controlled way
// - Make types fulfill the traits by supplying impl blocks
// - Define impl blocks for types that have generics too, although you have to be careful
//   about the type bounds
// - Type bounds allow us to narrow down our generic types by declaring what traits need
//   to be implemented for a tyoe


// Trait Bound - give boundries for generics.
// Inform the compiler that we are not defining all types
// but only a certain subset of them
// impl<T: Add<T, Output=T>> == this impl has a generic type T
// Add for Money == impl the Add trait for the type Money<T> where what T stands for was declared earlier
// T: Add == This type has to impl the Add Trait (if not, we can't use the + operator on it)
// <T, Output=T> == the impl of Add must have its input and output types the same
impl<T: Add<T, Output=T>> Add for Money<T> {
    type Output = Money<T>; //
    fn add(self, rhs: Money<T>) -> Self::Output {
        assert!(self.currency == rhs.currency);
        Money { currency: rhs.currency, amount: self.amount + rhs.amount }
    }
}

fn main() {
    let whole_euros_1 : Money<u8> = Money { amount: 42, currency: "EUR".to_string()};
    let whole_euros_2 : Money<u8> = Money { amount: 24, currency: "EUR".to_string()};
    let summed_euros = whole_euros_1 + whole_euros_2;

    println!("Summed Euros: {:?}", summed_euros);
}

