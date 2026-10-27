fn main(){

    let mut count = 1;

    while count <= 5 {
        println!("Count: {}",count);
        count += 1;
    }

    while count < 5 {
    println!("This won't be printed.");
    }

    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            break;  // also use continue for skipping this 
        }
        if num == 7 {
            num += 1;
            continue;
        }
        println!("Number: {}", num);
        num += 1;
    }

}