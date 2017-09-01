use std::fmt::Debug;

// Takaways:
// - Slices are views into existing and their size is more dynamic
// - Type is &[T]
// - Use slices to pass sequences to functions
// - An index trait can be used to make your own types indexable or sliceable
// - Syntax for slicing == [n..m] n is inclusive starting point; m is non-inclusive endpoint

fn print_slice<T: Debug>(slice: &[T]) {
    println!("{:?}", slice);
    println!("");
}

fn main() {
    let mut integer_array_1 = [1, 2, 3];
    let integer_array_2: [u64; 3] = [2, 3, 4];
    let integer_array_3: [u64; 32] = [0; 32];
    let integer_array_4: [i64; 16438] = [-5; 16438];
    integer_array_1[1] = 255;

    println!("integer_array_1{:?}", integer_array_1);
    println!("integer_array_2 {:?}", integer_array_2);
    println!("integer_array_3 {:?}", integer_array_3);
    // println!("integer_array_4 {}", integer_array_4);
    println!("integer_array_1[0] {}", integer_array_1[0]);
    println!("integer_array_1[5] {}", integer_array_1[2]);
    println!("========================================");

    // ------- Slice Usage -------

    let array: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Whole array just borrowed: ");
    print_slice(&array);

    println!("Whole array sliced: ");
    print_slice(&array[..]);

    println!("Without the first element: ");
    print_slice(&array[1..]);

    println!("One element from the middle: ");
    print_slice(&array[3..4]);

    println!("First Three Elements: ");
    print_slice(&array[..3]);

    // println!("Oops, going too far!: ");
    // print_slice(&array[...900]);
}
