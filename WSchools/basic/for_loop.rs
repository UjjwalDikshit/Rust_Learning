fn main(){
    for i in 1..6 { // excluding 6 , 1..=6, include 6
        println!("i is : {}",i);
    }

    for i in 1..=10 {
        if i == 3 {
            continue;
        }
        if i == 5 { 
            break;
        }
        println!("i is: {}",i)
    }
}
// Rust handles the counter variable (i) automatically, You don't need to declare or increment it manually.