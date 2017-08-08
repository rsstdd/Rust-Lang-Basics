// Import std mem
use std::mem;

fn main() {
    // "let" creates a binding - binds the variable a (a memory location) to 123
    // 8 bits in 1 byte - unsigned 8 bits
    // 8 bits with the value 123
    // Unsigned - whole numbers represented by signed and unsigned
    // Unsigned integer - is 0 or Positive from 0 - 255
    // i8 - would be a signed number; represents a signed integer - signed -127-128
    let a:u8 = 123;

    // Curley braces are replaced by the second argument to the println! macro
    println!("a = {}", a);

    // In Rust variable bindings are immutable by default, which means you cannot change the value.
    // You have to explicitly assign a value to be mutable
    // mutable:

    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    // You can get Rust to infer type (b:i8, for example)
    let mut c = 123456789; // 32-bit signed i32 (integer 32 bits)
    // Function from std::mem (import std::mem namespace) will output the mem size
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    //
    c = -1;
    println!("c = {} after modification", c);

    // i8 u8, i16 u16, i32 u32, i64 u64 <-- All datatypes that you can use in the program
    // // isize/usize - integral datatypes that correspond to the size of the pointer
    // If you had an address in memory, what would that size of that address be?
    // Depends on your system: 32bit sys = 32 bit variable; 64bit system = 62 bit variable
    // useful whn you need a variable whose size is the same size as a memory address as the system that you are running on
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z * 8); // size_of_z is in bytes - you need bits so * 8


    //  d:char optional - compiler will assign types
    let d:char = 'x';
    // d has the value of x, but the size is the larges unicode set that you can have so a single char is 4-bytes
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    // default behavior is double-percision value, 8 bytes or 64 bits and datatype is f64
    let e = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // To change you have to be explicit
    let ef32:f32 = 2.5;
    println!("ef32 = {}, sizef32 = {} bytef32s", ef32, mem::size_of_val(&ef32));
    // ef32 = 2.5, sizef32 = 4 bytef32s


    // Booleans -> single byte
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    // g = false, size = 1 bytes
    let f = 4 > 0;
    println!("{}", f);
}
