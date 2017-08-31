const MEANING_OF_LIFE:u8 = 42; // This variable has no fixed address

// static mut Z:i32 = 123; // avoid if possible

fn main() {
    // unsafe; // to use a static mutable variable
    println!("{}", MEANING_OF_LIFE);
}

//  You can have global variables in Rust Programs;
// this variable lives as long as the program runs
