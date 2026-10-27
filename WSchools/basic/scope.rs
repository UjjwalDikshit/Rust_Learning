// how variables act inside and outside of functions.
// Scope refers to where a variable is allowed to be used.
// In Rust, you can declare a new variable with the same name in the same scope using let. This is called shadowing:

fn main() {
    let x = 5; // no longer accessible
    let x = 10;

    println!("x is: {}", x); // prints 10

    let x = 5;

    {
    let x = 10;
    println!("Inside block: {}", x);
    }

    println!("Outside block: {}", x);
}
