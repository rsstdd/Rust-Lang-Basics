// String Slcies

const CONSTANT_STRING: &'static str = "This is a constant string";
// & - this is a refererence (point to existing strings)
// 'static - lifetime of the reference is static - Lives for the whole duration of program
// str - indicates string slice

fn say_hello(to_whom: &str) { // If you need to pass a string into a function use the str slice &str
  println!("Hey {}!", to_whom);
  println!("{}", "");
}

fn main() {
    // Rust type inference means that we can omit the types inside
    // the body of functions when it suits
    let another_string = "This string is local to the main function";

    println!("Constant: ==> {}", CONSTANT_STRING);
    println!("{}", "");
    println!("another_string: ==> {}", another_string);
    println!("{}", "");


    let str_slice: &'static str = "you";
    let string: String = str_slice.into(); // into() -> generic way to convert types

    say_hello(str_slice);
    say_hello(&string);
}

