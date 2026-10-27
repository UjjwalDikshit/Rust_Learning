fn say_hello(){ // simple function
    println!("Hello from a function!");
}

fn greet(name: &str){ // with parameter
    println!("Hello, {}!",name);
}

// fn add(a : i32, b:i32) -> i32{
//     return a + b;
// }

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn main(){
    say_hello();
    greet("Ujjwal");

    let sum = add(3,5);
    println!("Sum is: {}",sum);
}