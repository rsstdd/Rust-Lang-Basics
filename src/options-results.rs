use std::string::FromUtf8Error;

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

fn bytestring_to_string_with_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    match String::from_utf8(str) {
        Ok(str) => Ok(str.to_uppercase()),
        Err(err) => Err(err)
    }
}

fn bytestring_to_string(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(str).map(|s| s.to_uppercase())
}

fn main() {
    let faulty_bytestring = vec!(130, 131, 132, 133);
    let ok_bytestring = vec!(80, 82, 84, 85, 86);

    let sl_faulty = bytestring_to_string_with_match(faulty_bytestring.clone());
    let s1_ok = bytestring_to_string_with_match(ok_bytestring.clone());

    let s2_faulty = bytestring_to_string(faulty_bytestring.clone());
    let s2_ok = bytestring_to_string(ok_bytestring.clone());

    println!("");

    println!("Read the String: {:?}", sl_faulty);
    println!("Read the String: {:?}", s1_ok);

    println!("=============================");

    println!("Read the String: {:?}", s2_faulty);
    println!("Read the String: {:?}", s2_ok);
}
