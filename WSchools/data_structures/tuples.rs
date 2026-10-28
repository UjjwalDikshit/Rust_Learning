
fn get_user() -> (String,i32) {
    (String::from("Ujjwal Dikshit"),23)
}

fn main(){
    let person = ("Ujjwal Dikshit",30,true); // packing
    // This tuple contains a &str, an i32, and a bool.

    println!("Name is {}",person.0);
    println!("Age is {}",person.1);
    println!("Is Active is {}",person.2);

    // un_packing

    let (name,age,active) = person;

    println!("Name is {}",name);
    println!("Age is {}",age);
    println!("Is Active is {}",active);


    let user_info = get_user();
    println!("User: {} ( {} years old )",user_info.0,user_info.1);
}