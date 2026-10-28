// store key-value pairs
// import this standard library

use std::collections::HashMap;

fn main(){
    let mut capitalcities = HashMap::new();

    capitalcities.insert("England","London"); // key : pair
    capitalcities.insert("Germany","Berlin");
    capitalcities.insert("India","Delhi");
    capitalcities.insert("Norway","Oslo");

    println!( "{:?}",capitalcities.get("England") );

    if let Some(city) = capitalcities.get("England") {
        println!("The capital of England is {}.",city);
    }
    else {
        println!("England is not in the map");
    }

    capitalcities.insert("India","New Delhi"); // update value
    capitalcities.remove("England"); // remove key
    
    // loop through a hash_map

    for (country,city) in &capitalcities {
        println!("The capital of {} is {}",country,city);
    }

    println!("{:?}",capitalcities);

}