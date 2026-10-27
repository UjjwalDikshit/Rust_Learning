fn main(){
    // let x = 7;
    // let y = 5;

    let score = 85;

    if score >= 90 {
    println!("Grade: A");
    } else if score >= 80 {
    println!("Grade: B");
    } else if score >= 70 {
    println!("Grade: C");
    } else {
    println!("Grade: F");
    }

    let time = 20;
    let greeting  = 

    if time < 18 {
        "Good day."
    }else{
        "Good Evening."
    };

    println!("{}",greeting);

    // let number = 5;   # get error 
    // let result = if number < 10 { "Too small" } else { 100 };
    // println!("{}", result);

}

// When using if as an expression, you must include else. This ensures the result always has a value.
// Rust doen't have ternary operator.
// The value from if and else must be the same type, like two pieces of text or two numbers (in the example above, both are strings).
