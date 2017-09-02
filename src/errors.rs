use std::string::FromUtf8Error;
use std::thread;
use std::panic;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

// +------------------------+
// | Options/Result values  |
// +------------------------+

// map<U, F>(self, f: F) -> Result<U, E>
//     where F: FnOnce(T) -> U
// map<U, F>(self, f: F) -> Option<U>
//     where F: FnOnce(T) -> U

// map_err<F, O>(self, f: O) -> Result<T, F>
//     where O: FnOnce(E) -> F

// The Map Fns for both Option and Result types take a fn that:
//  - transforms a value of the type T to a val of type U (the FnOnce declaration)
//  - The return type tells us that the new value fo type U is wrapped inside the returned Result or Option
//  - Result - err type is left untouched
//  - map_err method the ok type is left untouched and the error type os mapped via the f function

// Use this when:
//  * your own library method when you'd like to modify the Ok/Some value but propagate any
//    Err or None values upwards to a caller

// fn bytestring_to_string_with_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
//     match String::from_utf8(str) {
//         Ok(str) => Ok(str.to_uppercase()),
//         Err(err) => Err(err)
//     }
// }

// fn bytestring_to_string(str: Vec<u8>) -> Result<String, FromUtf8Error> {
//     String::from_utf8(str).map(|s| s.to_uppercase())
// }

// fn main() {
//     let faulty_bytestring = vec!(130, 131, 132, 133);
//     let ok_bytestring = vec!(80, 82, 84, 85, 86);

//     let sl_faulty = bytestring_to_string_with_match(faulty_bytestring.clone());
//     let s1_ok = bytestring_to_string_with_match(ok_bytestring.clone());

//     let s2_faulty = bytestring_to_string(faulty_bytestring.clone());
//     let s2_ok = bytestring_to_string(ok_bytestring.clone());

//     println!("");

//     println!("Read the String: {:?}", sl_faulty);
//     println!("Read the String: {:?}", s1_ok);

//     println!("=============================");

//     println!("Read the String: {:?}", s2_faulty);
//     println!("Read the String: {:?}", s2_ok);
// }

// +------------+
// | try! Macro |
// +------------+

// With Match
// fn bytestring_to_string_with_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
//     let ret = match String::from_utf8(str) {
//         Ok(str) => str.to_uppercase(),
//         Err(err) => return Err(err)
//     };

//     println!("Conversion succeeded: {}", ret);
//     Ok(ret)
// }

// Try!
// bytestring_to_string_with_try(str: Vec<u8>) -> Result<String, FromUtf8Error> {
//     let ret = try!(String::from_utf8(str));
//     println!("Conversion succeeded: {}", ret);
//     Ok(ret)
// }

// The try! macro has a caveat. Since it expands to an early return (might return a Result or an Option)
// It cannot be used inside the main function at all.
// Because: the signature of the main function is this: fn main()
// It does not take any parameters and returns nothing, so it cannot return either an Option or a Result type
// Thus the compiler would think:
      // fn main() {
      //     let empty_ok_value = Ok(());
      //     try!(empty_ok_value);
      // }

// fn bytestring_to_string(str: Vec<u8>) -> Result<String, FromUtf8Error> {
//     String::from_utf8(str).map(|s| s.to_uppercase())
// }

// fn main() {
//     let faulty_bytestring = vec!(130, 131, 132, 133);
//     let ok_bytestring = vec!(80, 82, 84, 85, 86);

//     let sl_faulty = bytestring_to_string_with_match(faulty_bytestring.clone());
//     let s1_ok = bytestring_to_string_with_match(ok_bytestring.clone());

//     let s2_faulty = bytestring_to_string(faulty_bytestring.clone());
//     let s2_ok = bytestring_to_string(ok_bytestring.clone());

//     println!("");

//     println!("Read the String: {:?}", sl_faulty);
//     println!("Read the String: {:?}", s1_ok);

//     println!("=============================");

//     println!("Read the String: {:?}", s2_faulty);
//     println!("Read the String: {:?}", s2_ok);
// }

// +----------------+
// | The ? operator |
// +----------------+

// A shorter way of writing try! macros is available with the ? operator.
// bytestring_to_string_with_qmark(str: Vec<u8>) -> Result<String, FromUtf8Error> {
//     let ret = String::from_utf8(str)?;
//     println!("Conversion succeeded: {}", ret);
//     Ok(ret)
// }

// // This is useful if you have a combined statement of several operations,
// // where a failure in each operator should mean a failure of the whole.
// // Ex: open, read, and convert it to uppercase in a single line:
// // File::create("foo.txt")?.write_all(b"Hello world")

// fn bytestring_to_string(str: Vec<u8>) -> Result<String, FromUtf8Error> {
//     String::from_utf8(str).map(|s| s.to_uppercase())
// }

// fn main() {
//     let faulty_bytestring = vec!(130, 131, 132, 133);
//     let ok_bytestring = vec!(80, 82, 84, 85, 86);

//     let sl_faulty = bytestring_to_string_with_match(faulty_bytestring.clone());
//     let s1_ok = bytestring_to_string_with_match(ok_bytestring.clone());

//     let s2_faulty = bytestring_to_string(faulty_bytestring.clone());
//     let s2_ok = bytestring_to_string(ok_bytestring.clone());

//     println!("");

//     println!("Read the String: {:?}", sl_faulty);
//     println!("Read the String: {:?}", s1_ok);

//     println!("=============================");

//     println!("Read the String: {:?}", s2_faulty);
//     println!("Read the String: {:?}", s2_ok);
// }


// +-----------------+
// |    Panicking    |
// +-----------------+

// Non-recoverable errors that crash the thread.
// If the current thread is the main one, the program crashes.
// Panicking happens in the same process as exceptions:
//  - unwinding the call stack
//    - climbing up and out of the place in the code where the panicking happened to the top
//      then the thread quits.

// fn f1() -> thread::JoinHandle<()> {
//     thread::spawn(move || {
//         f2();
//     })
// }

// fn f2() {
//     f3();
// }

// fn f3() {
//     panic!("Panicking in f3!");
// }

// fn main() {
//     let child = f1();
//     child.join().ok();

//     f2();
//     println!("This is unreachable code.");
// }

// =>

// thread '<unnamed>' panicked at 'Panicking in f3!', errors.rs:179
// note: Run with `RUST_BACKTRACE=1` for a backtrace.
// thread 'main' panicked at 'Panicking in f3!', errors.rs:179

// You'll want to fix errors via the Option/Result mechanism
// You can use this method to handle fatal errors in worker threads; let the workerrs die, and restart them.



// +--------------------+
// |    Catch Unwind    |
// +--------------------+

// std::panic::catch_unwind
// Offers more control over how panics are handled
// Takes a closure and handles any panics that happen inside it.

// fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> // Type signature

// The return value of catch_unwind has an additional constraint; UnwindSafe
// - variables in the closure are exception safe. most are but not &mut T


// fn main() {
//     panic::catch_unwind(|| {
//         panic!("Panicking!");

//     }).ok();
//     println!("Survived the Panic")
// }

// =>

// thread 'main' panicked at 'Panicking!', errors.rs:212
// note: Run with `RUST_BACKTRACE=1` for a backtrace.
// Survived the Panic

// +-------------------+
// |    Error Trait    |
// +-------------------+

// pub trait Error: Debug + Display + Reflect {
//     fn description(&self) -> &str;
//     fn cause(&self) -> Option<&Error> { None }
// }

// Two methods for this error type:
// 1.) description - returns a string slice references; tells in fre form what the error is about
// 2.) cause - returns an optional reference to another Error trait, representing a possible lower-level
//     reason for the error. The highest-level Error trait has access to the lowest level, making a precise
//     logging of the error possible.

// #[derive(Debug)]
// enum Currency { USD, EUR }

// #[derive(Debug)]
// struct CurrencyError;

// impl Currency {
//     fn new(Currency: &str) -> Result<Self, CurrencyError> {
//         match Currency {
//             "USD" => Ok(Currency::USD),
//             "EUR" => Ok(Currency::EUR),
//             _ => Err(CurrencyError{})
//         }
//     }
// }

// #[derive(Debug)]
// struct Money {
//     currency: Currency,
//     amount: u64
// }

// #[derive(Debug)]
// struct MoneyError;

// impl Money {
//     fn new(currency: &str, amount: u64) -> Result<Self, MoneyError> {
//         let currency = match Currency::new(currency) {
//             Ok(c) => c,
//             Err(_) => panic!("Unimplemented!")
//         };

//         Ok(Money {
//             currency: currency,
//             amount: amount
//         })
//     }
// }

// fn main() {
//     let money_1 = Money::new("EUR", 12345);
//     let money_2 = Money::new("FIM", 600000);

//     println!("Money_1 is {:?}", money_1);
//     println!("Money_2 is {:?}", money_2);
// }

// +---------------------+
// |    Error Trait_2    |
// +---------------------+

#[derive(Debug)]
enum Currency {
  USD,
  EUR
}


#[derive(Debug)]
struct CurrencyError { // no low-level cause for error - Return None
    description: String
}
impl Display for CurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CurrencyError: {}", self.description)
    }
}

impl Error for CurrencyError { // no low-level cause for error - Return None
    fn description(&self) -> &str {
        "CurrencyError"
    }
}

impl Currency {
    fn new(currency: &str) -> Result<Self, CurrencyError> {
        match currency {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            _ => Err(CurrencyError{ description: format!("{} not a valid currency", currency)})
        }
    }
}

#[derive(Debug)]
struct Money {
    currency: Currency,
    amount: u64
}

#[derive(Debug)]
struct MoneyError {
    cause: CurrencyError
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoneyError due to {}", self.cause)
    }
}

impl Error for MoneyError {
    fn description(&self) -> &str {
        "MoneyError"
    }
    fn cause(&self) -> Option<&Error> {
        Some(&self.cause)
    }
}

impl Money {
    fn new(currency: &str, amount: u64) -> Result<Self, MoneyError> {
        let currency = match Currency::new(currency) {
            Ok(c) => c,
            Err(e) => return Err(MoneyError { cause: e })
        };

        Ok(Money {
            currency: currency,
            amount: amount
        })
    }
}

fn main() {
    let money_1 = Money::new("EUR", 12345);
    let money_2 = Money::new("FIM", 600000);

    println!("Money_1 is {:?}", money_1);
    println!("Money_2 is {:?}", money_2);

    let cause_for_money_2 = money_2.unwrap_err();
    println!("{}", cause_for_money_2);
}
