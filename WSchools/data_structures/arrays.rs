

fn main(){
    let numbers = [1,2,3,4,5];
    println!("The first number is: {}",numbers[2]);
    // println!("The first number is: {}",numbers[6]);

    // To change the value of a specified element, refer to the index number and assign a new value.

    let mut numbers = [3,4,24,45];
    numbers[0] = 232;
    println!("The new first number is: {}", numbers[0]);
    println!("This array has {} elements.",numbers.len());

    let fruits = ["apple", "banana", "orange"];
    for fruit in fruits {
        println!("I like {}",fruit);
    }

    println!("{:?}",numbers); // printing whole array, use {:?} inside println!
    // {:?} for data structures like arrays and vectors - when printing the entire structure.



}