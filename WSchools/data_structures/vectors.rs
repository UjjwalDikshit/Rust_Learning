// for resizable arrays use vectors,grow and shrink, use vec! macro to create a vector

fn main(){
    let mut cars = vec!["Volvo","BMW","Ford"];
    cars.push("Mazda");
    cars[0] = "Tata";
    cars.pop();
    // add or remove elements at specified index
    cars.insert(0,"Mazda");// shift other to next index
    cars.remove(0);
    println!("{:?}",cars);
    println!("Size of cars vector: {}",cars.len());

    // iterate through vector

    for car in &cars{ // & -> reference so that next time also use it.
        println!("I like {}.",car);
    }

    println!("{:?}",cars);

}