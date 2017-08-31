// String
// - Unicode
// - Mutable and Growable
// - Can be created during runtime
// - Actually constains data

// Has to be allocated in the Heap
// - possibly reallocated on growth

// String types can be cast into &str - Not vice cersa
// to do cast, you have to be explicit:
//    let str_slice: &'static str = "you";
//    let string: String = str_slice.into();
//    let string: String = string_slice.into();

// // Build and Manupulate Strings
// Sting::new() // allocated an empty string type
// String::from(&str) // allocates a new string type and populates it from a string slice
// String::with_capacity(capacity: usize) // Allocates an empty Str w/ a preallocated size
// String::from_utf8(vec: Vec<u8>) //tries to allocate new String from bytestring must be UTF8

// The len() method gives you the length of the String, taking unicode into account
//   - unicode length not byte length

// The push(ch: char) and push_str(string: &str) methods add a char or a string slice to the String

fn main() {

    let mut empty_string = String::new();
    let empty_string_with_capacity = String::with_capacity(50);
    let string_from_bytestring: String = String::from_utf8(vec![82, 85, 83,
    84]).expect("Creating String from bytestring failed");

    println!("empty_string.len() {}", empty_string.len());
    println!("empty_string_with_capacity.len() {}", empty_string_with_capacity.len());
    println!("string_from_bytestring {}", string_from_bytestring.len());

    println!("Bytestring says {}", string_from_bytestring);

    empty_string.push('1');
    println!("1) empty string now contains {}", empty_string);

    empty_string.push_str("2345");
    println!("2) empty string now contains {}", empty_string);

    println!("3) The length of empty_string is now {}", empty_string.len())

}
