// Rust has three types of loops: loop, while, and for.
// loop is the simplest of Rust's three loop types.
// It will run forever unless you tell it to stop:
// press Ctrl + C to end the program.

fn main(){
    // loop {
    //     println!("this will repeat forever!");
    // }

    // To stop a loop, use the break keyword:

    let mut count = 1;

    // loop {
    //     println!("Helllo World!");

    //     if count == 3 {
    //         break;
    //     }

    //     count += 1;
    // }

    // You can also return a value from a loop using break with a value.

    let result = loop {
        println!("Hello!");

        if count == 3 {
            break count;
        }

        count += 1;
    };

    println!("The loop stopeed at: {}",result);

}



