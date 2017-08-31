// variables within blocks only exist in scope
    // where they are declared
    // Two Kinds:
    // Fn scope
    // block scope

fn main() {
    let a = 123;
    println!("a = {}", a,);

    {
        let b = 156;
        let a  = 777; // different variable with the same name
        println!("inside b has = {}", b);

        println!("inside a has = {}", a);
    }
}

// Shadow: inner scopes have access to outer variables
