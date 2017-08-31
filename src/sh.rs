#![allow(dead_code)]
#![allow(unused_variable)]

mod sh;
use std::mem;

struct Point {
  x: f64,
  y: f64
}

fn origin() -> Point {
  Point{ x: 0.0. y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin(); // stack allocated
    let p2 = Box::new(origin()); // heap allocated - pointed to mem location

    println!("p1 takes up {} bytes", mem::size_of_val(&p1))
    // println!("p1 takes up {} bytes", mem::size_of_val(&p2))
}
// Stack and Heap

// Stack

// simple bindings
// let x = 5; // i32 - 4 bytes somewhere

// stack is the bottom part ot the memory store
// When you allocate, you assign a chunk of memory that are then removed when not needed
// These variables contain the actual values
// LIFO structure
// Used when you call functions (call stack)


// Heap

// Chunk of memory where you can allocate things for long-term storage
// top of memory store
// let x = Box::new(5); // --> allocate things for longer storage
// This creates a reference/pointer to the heap.
// To reference this in memory (follow the memory address)
// println!("{}", *x); // dereferencing operator


