// Constants and Statics

// Mutable statics can only be used inside unsafe blocks
// Immutable statics can be used everywhere

// Prefer using constants if you don't don't need the memory location of your global values

const THE_ANSWER: u32 = 42;
static mut meep: u32 = 4;
static FUUP: u8 = 9;

fn main() {
    unsafe {
        println!("Meep is {}", meep);
        meep = 42;
        println!("Fuup is {}", FUUP);
    }
}

