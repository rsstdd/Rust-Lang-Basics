// Generic Types

// Takaways:
// - Syntax for generic types is <T>, where T is any valid Rust type
// - In every Block that it is used, it needs to be declared
// - Option - in the std lib, is used to represent any value that might be nothing
// - Result - represents operations that may or may not succeed

// Use Option/Result when you need vals that are operationally empty or when you have operations that
// might or might not suceed
// - Have methods:
//   * is_some returns true if the Option type has a value, otherwise false
//   * is_none works the same as is_some but not vice versa
//   * expect extracts the value from inside the Option type or panics if it was none

enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
  Ok(T),
  Err(E)
}

#[derive(Debug)]
struct Money<T> {
    amount: T,
    currency: String
}

fn select_first<T>(p1: T, _: T) -> T {
    p1
}

fn main() {
    let whole_euroes : Money<u8> = Money { amount: 42, currency: "EUR".to_string()};
    let floating_euros : Money<f32> = Money { amount: 24.234, currency: "EUR".to_string()};

    println!("Whole Euros: {:?}", whole_euroes);
    println!("Floating Eros: {:?}", floating_euros);

    let x = 1;
    let y = 2;

    let a = "Sheep";
    let b = "Wolf";

    println!("Selected first {}", select_first(x, y));
    println!("Selected first {}", select_first(a, b));
}
