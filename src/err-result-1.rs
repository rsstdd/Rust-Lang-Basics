use std::io::Read;
use std::path::Path;
use std::fs::File;

// fn main()  {
//     let path = Path::new("data.txt");
//     let file = File::open(&path);
//     let mut s = String::new();
    // file.read_to_string(&mut s);

    // println!("Read the String: {}", s);
// }

// =>

// error[E0599]: no method named `read_to_string` found for type `std::result::Result<std::fs::File, std::io::Error>` in the current scope
//  --> err-result-1.rs:9:10
//   |
// 9 |     file.read_to_string(&mut s);
//   |          ^^^^^^^^^^^^^^

// error: aborting due to previous error(s)

// ===========================================================================|
// See the result type in the Error? The Type is actually Result<File, Error>
// type Result<T> = Result<T, std::io::Error>; --> Result type used by all IO operations
// The type is defined in std::io::Result module

// fn main() {
//     let path = Path::new("data.txt");
//     let mut file = match File::open(&path) {
//         Ok(file) => file,
//         Err(err) => {
//             println!("Error while opening file: {}", err);
//             panic!();
//         }
//     };
//     let mut s = String::new();
//    let _ = file.read_to_string(&mut s); // var assignment tells the compiler to ignore the return val

//     println!("Read the String: {}", s);
// }

// fn read_to_string(&mut self, bug: &mut String) -> Result<usize>
// * &mut self == the variable we are calling this method on needs to be mutable
//   - (Reading the filechanges internal pointers of the file handle)
// * The Ok and Err case by returning the actual file handle if everything is Ok and bailing if not

// This Becomes Tedious! Enter helper methods from std lib
// * Use them to simplify error cases where you really do not expect things to fail
// * unwrap(self): T - expects self to be Ok/Some; -> value contained within. If Err/None, it panics
// * expect(self, msg: &str): T - like unwrap; Outputs custom error before paniking in addition to Err contents
// * unwrap_or(self, opt_b: T): T - Like unwrap; Instead of paniking, -> opt_b
// * unwrap_or_else(self, op: F): T - Like unwrap; Instead of paniking, it calls op, which needs to be a fn or
//     - closure: more precisely, anything that implements FnOnce trait.


fn main() {
    let path = Path::new("data.txt");
    let mut file = File::open(&path)
      .expect("Error while opening data.txt");

    let mut s = String::new();
    file.read_to_string(&mut s)
      .expect("Error while readign file contents");

    println!("Read the string: {}", s);
}

// Much better, isn't it? - Only use when the error is so critical that panicking is a good option
