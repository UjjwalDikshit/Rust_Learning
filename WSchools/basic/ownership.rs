/*
Rust uses "ownership" to manage memory in a safe way.
Every value in Rust has an owner. The owner is usually a variable.

Each value has one owner
When the owner goes out of scope, the value is deleted
You can only have one owner at a time, unless you borrow it (covered in the next chapter)

But simple types like numbers, characters and booleans are copied, not moved.

This means you can still use the original variable after assigning it to another:
However, if you don't need to own the value twice, using a reference (&) is usually better than cloning
*/

fn main() {
    //In this example, a owns the string. Then we move it to b:

    let a = String::from("Hello");
    let b = a;
    let c = b.clone();// Now both b and c have same vaulue

    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value
    println!("{}", c); // works

    let a = 5;
    let b = a; // value copied, not ownership moved.
    println!("a = {}", a); // Works
    println!("b = {}", b); // Works


}
