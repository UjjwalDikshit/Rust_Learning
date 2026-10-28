/*
Sometimes you want to use a value without taking ownership of it.
Rust lets you do this using a reference - this is called borrowing:


*/

fn main(){
    let a = String::from("Hello");
    let b =&a;

    println!("a = {}",a);
    println!("b = {}",b);

    // If you want to change a value through a reference, you need to make the reference mut:

    let mut name = String::from("Ujjwal");
    let name_ref = &mut name; // Note: You can only have one mutable reference to a value at a time!

    name_ref.push_str(" Dikshit");
    // name.push_str(" Ji"); error
    // only name_ref can write, name can only read. as for some time ownership is in hand of this variable reference.

    println!("{}",name_ref);
}