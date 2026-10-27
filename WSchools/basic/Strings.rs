fn main(){

    let greeting: &str = "Hello";
    println!("{}",greeting);

    let text1 = "Hello World".to_string();
    let text2 = String::from("Hello World");

    let mut greeting = String::from("Hello");
    greeting.push_str("World"); // to add text to string
    println!("{}",greeting);

    let mut word = String::from("Hi"); 
    word.push('!'); // add character
    println!("{}",word);

    // concatenate strings: You can combine strings using the format! macro:

    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");

    let result = format!("{} {} {}",s1,s2,s3);
    println!("{}",result);

    // You can also use the + operator to combine strings, but it can get messy with many values.
    // You can only add a &str to a String with +. That is why &s2 and &s3 (a string slice) is used here.
    let result = s1 + " " + &s2 + " " + &s3;
    println!("{}", result);

    let name = String::from("John");
    println!("Length: {}",name.len());
    

}

// &str - is called "string slices", and is used for fixed text like "Hello"
// String - used when you need a string that can change
// String type because it is more flexible and can be changed over time.
// You can create a String from a string literal using the to_string() method or the String::from() function:

