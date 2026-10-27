// variables in Rust do not need to be declared with a specified type (like "String" for text or "Int" for numbers, if you are familiar with those from C or Java).
// automatically variable is decided.

// _ initially at name tells to ignore if not used in code.

fn main(){
    // let _my_num = 5; 
    // let _my_double = 5.99;
    // let _my_letter = 'D';
    // let _my_bool = true;
    // let _my_text = "Hello";

    // let my_num = 5;         // integer
    // let my_double = 5.99;   // float
    // let my_letter = 'D';    // character
    // let my_bool = true;     // boolean
    // let my_text = "Hello";  // string

    // let my_num: i32 = 5;
    // let my_second_num: u32  = 3;
    // let my_double : f64 = 5.99;

    // let my_letter : char  = 'D';
    // let my_bool : bool = true;
    // let my_string : &str = "Hello";

    let temperature : i32 = -5;
    let age: u32 = 23;

    println!("Temperature: {}",temperature);
    println!("Age: {}",age);

    let name: &str = "John";
    println!("Hello, {}",name);

}