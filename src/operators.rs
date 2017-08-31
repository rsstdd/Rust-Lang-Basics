use std::mem;
// Operators

fn main() {
    // arithmetic
    // operators have precidence
    let mut a = 2 + 3 * 4; // +-*/
    println!("{}", a);

    a += 1; // does not support ++ or --
    println!("a = {}", a);

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    // There is no Power operator; You'll have to use the built-in functions
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // integer (whole number not float)
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // Pi is a constant for the f64 type; Constance are in caps
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    // only for integers
    let c = 1 | 2; //  | OR  -==-  & AND  -==-  ^  XOR -==- ! BANG
                    // 01 OR 10 == 11 ==3_10
    println!("1|2 = {}", c);
    // Shift
    let two_to_ten = 1 << 10;
    println!("2^10 = {}", two_to_ten);

    // Logical Operators
    let pi_less_4 = std::f64::consts::PI < 4.0; // < <= > >= ==
    println!("isPiLessThan? {}", pi_less_4)

    let x = 5;
    let x_is_5 = x == 5; // true
}
